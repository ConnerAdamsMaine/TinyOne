use std::path::{Path, PathBuf};
use std::{env, fs};

const WORKSPACE_DIR_ENV: &str = "CARGO_WORKSPACE_DIR";

/// True when `text` is a workspace `Cargo.toml` (`[workspace]` table).
fn cargo_toml_declares_workspace(text: &str) -> bool {
    text.lines().any(|line| {
        let trimmed = line.trim();
        trimmed == "[workspace]" || trimmed.starts_with("[workspace.")
    })
}

fn is_workspace_root(dir: &Path) -> bool {
    let cargo = dir.join("Cargo.toml");
    cargo.is_file() && fs::read_to_string(&cargo).is_ok_and(|text| cargo_toml_declares_workspace(&text))
}

/// Walk `start` and its parents until a `Cargo.toml` with `[workspace]` is found.
fn walk_to_workspace(start: &Path) -> Result<PathBuf, String> {
    let mut dir = start.canonicalize().unwrap_or_else(|_| start.to_path_buf());
    loop {
        if is_workspace_root(&dir) {
            return Ok(dir);
        }
        match dir.parent() {
            Some(parent) => dir = parent.to_path_buf(),
            None => {
                return Err(format!(
                    "no Cargo.toml with [workspace] above {} (set {WORKSPACE_DIR_ENV} to override)",
                    start.display()
                ));
            }
        }
    }
}

/// Workspace root for tests.
///
/// Order: `CARGO_WORKSPACE_DIR` if it points at a workspace, else walk from
/// `manifest_dir` looking for a `Cargo.toml` that declares `[workspace]`.
///
/// # Panics
/// If neither the env var nor the directory walk finds a workspace root.
#[must_use]
pub fn repo_root_from_manifest(manifest_dir: &str) -> PathBuf {
    find_workspace_root(Path::new(manifest_dir)).unwrap_or_else(|err| panic!("{err}"))
}

fn find_workspace_root(start: &Path) -> Result<PathBuf, String> {
    if let Ok(dir) = env::var(WORKSPACE_DIR_ENV) {
        let candidate = PathBuf::from(dir);
        if is_workspace_root(&candidate) {
            return candidate.canonicalize().or(Ok(candidate));
        }
    }
    walk_to_workspace(start)
}

/// Workspace `target/<profile>` directory.
///
/// Prefers `CARGO_TARGET_DIR`, then `<workspace>/target/<profile>`, then
/// the crate-local `target/<profile>` for non-workspace builds.
#[must_use]
pub fn workspace_target_dir(manifest_dir: &str, profile: &str) -> PathBuf {
    if let Ok(dir) = env::var("CARGO_TARGET_DIR") {
        return Path::new(&dir).join(profile);
    }
    let workspace = repo_root_from_manifest(manifest_dir).join("target").join(profile);
    let crate_local = Path::new(manifest_dir).join("target").join(profile);
    if workspace.exists() || !crate_local.exists() {
        workspace
    } else {
        crate_local
    }
}

/// Asserts that a `Result` is `Err` and that the error string contains `needle`.
///
/// # Panics
/// If the result is `Ok(_)` or if the error string does not contain `needle`.
pub fn expect_error_contains<T, E: std::fmt::Display>(result: Result<T, E>, needle: &str) {
    let error = match result {
        Ok(_) => panic!("operation should fail"),
        Err(error) => error.to_string(),
    };
    assert!(error.contains(needle), "expected error to contain {needle:?}, got {error:?}");
}

#[cfg(test)]
mod tests {
    use super::{cargo_toml_declares_workspace, repo_root_from_manifest};

    #[test]
    fn detects_workspace_table() {
        assert!(cargo_toml_declares_workspace("[workspace]\nmembers = []\n"));
        assert!(cargo_toml_declares_workspace("[workspace.package]\nversion = \"0\"\n"));
        assert!(!cargo_toml_declares_workspace("[package]\nname = \"x\"\n"));
        assert!(!cargo_toml_declares_workspace("# [workspace]\n[package]\n"));
    }

    #[test]
    fn finds_workspace_from_this_crate() {
        let root = repo_root_from_manifest(env!("CARGO_MANIFEST_DIR"));
        let cargo = root.join("Cargo.toml");
        let text = std::fs::read_to_string(&cargo).expect("read workspace Cargo.toml");
        assert!(cargo_toml_declares_workspace(&text), "{}", cargo.display());
    }
}
