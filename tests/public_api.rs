//! Installed wrapper identity, argument forwarding and exit status contracts.

use std::env;

use async_fs_io::{
    TempDir,
    write_bytes,
};
use tokio::process::Command;

#[tokio::test]
async fn help_and_version_do_not_require_a_nightly_installation() {
    for option in ["--help", "--version"] {
        let result = Command::new(env!("CARGO_BIN_EXE_cargo-nightly"))
            .args(["nightly", option])
            .env("PATH", "")
            .output()
            .await
            .expect("run installed command");
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let text = String::from_utf8(result.stdout).expect("command text");
        if option == "--version" {
            assert_eq!(
                text,
                format!(
                    "cargo-nightly {}.{}.{}\n",
                    env!("CARGO_PKG_VERSION_MAJOR"),
                    env!("CARGO_PKG_VERSION_MINOR"),
                    env!("CARGO_PKG_VERSION_PATCH")
                )
            );
        } else {
            assert!(text.contains("cargo nightly"));
        }
    }
}

#[tokio::test]
async fn bare_direct_invocation_is_a_usage_error_without_a_panic() {
    let result = Command::new(env!("CARGO_BIN_EXE_cargo-nightly"))
        .output()
        .await
        .expect("bare command");
    assert_eq!(result.status.code(), Some(2));
    assert!(!String::from_utf8_lossy(&result.stderr).contains("panicked"));
}

#[tokio::test]
async fn forwarding_preserves_arguments_and_the_child_exit_code() {
    let directory = TempDir::create(env::temp_dir()).await.expect("fixture");
    let source = directory.path().join("cargo.rs");
    write_bytes(
        &source,
        br#"fn main() {
        for argument in std::env::args().skip(1) { println!("arg={argument}"); }
        std::process::exit(37);
    }"#,
    )
    .await
    .expect("fake Cargo source");
    let executable = directory
        .path()
        .join(if cfg!(windows) { "cargo.exe" } else { "cargo" });
    let compilation = Command::new("rustc")
        .arg(&source)
        .arg("-o")
        .arg(&executable)
        .output()
        .await
        .expect("compile fake Cargo");
    assert!(
        compilation.status.success(),
        "{}",
        String::from_utf8_lossy(&compilation.stderr)
    );
    let paths = env::join_paths(
        std::iter::once(directory.path().to_path_buf())
            .chain(env::split_paths(&env::var_os("PATH").expect("PATH"))),
    )
    .expect("fixture PATH");
    let result = Command::new(env!("CARGO_BIN_EXE_cargo-nightly"))
        .args(["nightly", "clippy", "--", "-D warnings"])
        .env("PATH", paths)
        .output()
        .await
        .expect("forward command");
    assert_eq!(result.status.code(), Some(37));
    assert_eq!(
        String::from_utf8(result.stdout)
            .expect("forwarded arguments")
            .replace("\r\n", "\n"),
        "arg=+nightly\narg=clippy\narg=--\narg=-D warnings\n"
    );
}
