use std::path::Path;
use std::process::Command;

#[test]
fn release_build_emits_c_linkable_library_artifacts() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    let output = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(manifest_dir)
        .output()
        .expect("release build should execute");

    assert!(
        output.status.success(),
        "release build failed\nstatus: {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    // With a workspace, `cargo build` from a crate dir outputs to the workspace
    // `target/release` (repo root), not `crates/tinyone_ralloc/target/release`.
    // Check both locations; prefer crate-local if present for non-workspace builds.
    let crate_release = Path::new(manifest_dir).join("target/release");
    // actual rlib/cdylib artifacts for crate-type = ["rlib", "cdylib"]
    let expected_static = if cfg!(windows) { "ralloc.lib" } else { "libralloc.rlib" };
    let expected_shared = if cfg!(windows) {
        "ralloc.dll"
    } else if cfg!(target_os = "macos") {
        "libralloc.dylib"
    } else {
        "libralloc.so"
    };
    let workspace_release = Path::new(manifest_dir).join("../../target/release");
    let release_dir = if crate_release.join(expected_static).is_file() {
        crate_release
    } else {
        workspace_release
    };
    assert!(
        release_dir.join(expected_static).is_file(),
        "missing C static library at {}",
        release_dir.join(expected_static).display()
    );
    assert!(
        release_dir.join(expected_shared).is_file(),
        "missing C shared library at {}",
        release_dir.join(expected_shared).display()
    );
}
