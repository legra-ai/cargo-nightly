//! Cargo subcommand to forward commands to the nightly toolchain.
//!
//! This extension enables nightly toolchain usage in cargo aliases by
//! providing a `cargo-nightly` subcommand that forwards all arguments to
//! `cargo +nightly <args>`.
//!
//! ## Why it exists
//!
//! Cargo aliases in `.cargo/config.toml` cannot directly use `+nightly`
//! syntax because cargo aliases can only invoke cargo subcommands, not
//! external commands or toolchain selectors. This plugin provides a
//! `cargo-nightly` subcommand that forwards all arguments to
//! `cargo +nightly <args>`, making it possible to use nightly toolchain
//! commands through cargo aliases.
//!
//! ## Usage
//!
//! Once installed, you can use it directly:
//!
//! ```bash
//! cargo nightly clippy
//! cargo nightly check
//! ```
//!
//! Or through aliases in `.cargo/config.toml`:
//!
//! ```toml
//! [alias]
//! clippy2 = "nightly clippy"
//! check2 = "nightly clippy"
//! ```

use std::env;
use std::process::{
    Command,
    Stdio,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Skip the first argument (program name) and the second argument (nightly)
    // Forward only the remaining arguments to cargo +nightly
    let cargo_args = &args[2..];

    // Build the command: cargo +nightly <args>
    let mut cmd = Command::new("cargo");
    cmd.arg("+nightly");
    cmd.args(cargo_args);

    // Forward stdin, stdout, stderr
    cmd.stdin(Stdio::inherit());
    cmd.stdout(Stdio::inherit());
    cmd.stderr(Stdio::inherit());

    // Execute the command and exit with the same code
    let status = cmd.status().expect("Failed to execute cargo +nightly");
    std::process::exit(status.code().unwrap_or(1));
}
