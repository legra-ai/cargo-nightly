//! Cargo subcommand forwarding explicit nightly work to `cargo +nightly`.
//! Use a repository's pinned compiler directly for reproducible CI.

use std::env;
use std::ffi::OsStr;
use std::process::{
    Command,
    Stdio,
};

const USAGE: &str = "Usage: cargo nightly <COMMAND> [ARGS]...\n\nForward COMMAND and ARGS to cargo +nightly.\n  --help     Show wrapper help without invoking a toolchain\n  --version  Show the installed wrapper version\n\nUse -- before flags that should be forwarded directly to Cargo.";

fn usage_error() -> ! {
    eprintln!("{USAGE}");
    std::process::exit(2);
}

fn main() {
    let mut arguments = env::args_os().skip(1).peekable();
    if arguments.next().as_deref() != Some(OsStr::new("nightly")) {
        usage_error();
    }
    let Some(command) = arguments.next() else {
        usage_error();
    };
    if command == "--help" || command == "-h" {
        if arguments.peek().is_some() {
            usage_error();
        }
        println!("{USAGE}");
        return;
    }
    if command == "--version" || command == "-V" {
        if arguments.peek().is_some() {
            usage_error();
        }
        println!("cargo-nightly {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    let mut child = Command::new("cargo");
    child.arg("+nightly");
    if command == "--" {
        if arguments.peek().is_none() {
            usage_error();
        }
    } else {
        child.arg(command);
    }
    child
        .args(arguments)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    match child.status() {
        Ok(status) => std::process::exit(status.code().unwrap_or(1)),
        Err(error) => {
            eprintln!("Failed to execute cargo +nightly: {error}");
            std::process::exit(1);
        }
    }
}
