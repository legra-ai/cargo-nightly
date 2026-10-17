# cargo-nightly

[![Crates.io](https://img.shields.io/crates/v/cargo-nightly.svg)](https://crates.io/crates/cargo-nightly)
[![Documentation](https://docs.rs/cargo-nightly/badge.svg)](https://docs.rs/cargo-nightly)
[![CI](https://github.com/legra-ai/cargo-nightly/actions/workflows/ci.yml/badge.svg)](https://github.com/legra-ai/cargo-nightly/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![Downloads](https://img.shields.io/crates/d/cargo-nightly.svg)](https://crates.io/crates/cargo-nightly)

Cargo subcommand that forwards commands to the nightly toolchain, enabling
nightly toolchain usage in cargo aliases.

## Why it exists

Cargo aliases in `.cargo/config.toml` cannot directly use `+nightly` syntax
because cargo aliases can only invoke cargo subcommands, not external
commands or toolchain selectors. This plugin provides a `cargo-nightly`
subcommand that forwards all arguments to `cargo +nightly <args>`, making
it possible to use nightly toolchain commands through cargo aliases.

### The Problem

If you try to use `+nightly` directly in a cargo alias:

```toml
[alias]
clippy2 = "+nightly clippy"  # ❌ This doesn't work
```

You'll get an error:

```text
error: no such command: `+nightly`
help: invoke `cargo` through `rustup` to handle `+toolchain` directives
```

### The Solution

With `cargo-nightly` installed, you can create aliases like:

```toml
[alias]
clippy2 = "nightly clippy"
check2 = "nightly clippy"
clippy-all = "nightly clippy --workspace --all-targets --all-features -- -D warnings"
```

When you run `cargo clippy2`, cargo will:

1. Look for a `cargo-nightly` subcommand
2. `cargo-nightly` forwards to `cargo +nightly clippy`
3. The nightly toolchain runs your command

## Installation

### Using cargo-binstall (Recommended)

First install cargo-binstall if you haven't already:

```bash
cargo install cargo-binstall
```

Then install cargo-nightly:

```bash
cargo binstall cargo-nightly
```

### Using cargo install

```bash
cargo install cargo-nightly
```

## Usage

Once installed, you can use it directly:

```bash
cargo nightly clippy
cargo nightly build
cargo nightly build
```

Or through aliases in `.cargo/config.toml`:

```toml
[alias]
# Use nightly clippy for comprehensive linting
clippy2 = "nightly clippy"
check2 = "nightly clippy"

# Comprehensive clippy checks
clippy-all = "nightly clippy --workspace --all-targets --all-features -- -D warnings"
clippy-fix = "nightly clippy --fix --workspace --all-targets --all-features --allow-dirty --allow-staged"
clippy-strict = "nightly clippy --workspace --all-targets --all-features -- -D warnings -D clippy::pedantic"

# Quick fixes
fixlib = "nightly clippy --fix --lib --allow-dirty"
```

Then use the aliases:

```bash
cargo clippy2
cargo check2
cargo clippy-all
```

## How it works

The `cargo-nightly` command is a simple wrapper that:

1. Takes all arguments after `nightly`
2. Executes `cargo +nightly <args>`
3. Forwards stdin, stdout, and stderr
4. Exits with the same exit code

This allows cargo to recognize `nightly` as a subcommand, which can then be
used in aliases.

## License

Copyright © 2026 DataRoad Inc, Delaware, USA, trading as Legra.

Licensed under either the [MIT license](LICENSE-MIT) or the
[Apache License, Version 2.0](LICENSE-APACHE), at your option.

## Reproducible toolchains

This wrapper deliberately selects floating `+nightly` for explicit
nightly work. CI and normal builds should invoke Cargo directly with
the repository's `rust-toolchain.toml` pin. The wrapper is unnecessary
when that file already selects the intended compiler.

`cargo nightly --help` and `cargo nightly --version` describe the
installed wrapper without invoking Rustup. To query the selected
Cargo instead, use `cargo nightly -- --version`. Command arguments
and nonzero child exit codes are preserved.
