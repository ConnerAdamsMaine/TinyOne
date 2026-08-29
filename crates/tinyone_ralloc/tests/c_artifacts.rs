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
    let workspace_release = Path::new(manifest_dir).join("../../target/release");
    let release_dir = if crate_release
        .join(if cfg!(windows) { "ralloc.lib" } else { "libralloc.a" })
        .is_file()
    {
        crate_release
    } else {
        workspace_release
    };
    let static_library = if cfg!(windows) { "ralloc.lib" } else { "libralloc.a" };
    let shared_library = if cfg!(windows) {
        "ralloc.dll"
    } else if cfg!(target_os = "macos") {
        "libralloc.dylib"
    } else {
        "libralloc.so"
    };
    assert!(
        release_dir.join(static_library).is_file(),
        "missing C static library at {}",
        release_dir.join(static_library).display()
    );
    assert!(
        release_dir.join(shared_library).is_file(),
        "missing C shared library at {}",
        release_dir.join(shared_library).display()
    );
}
