# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when
working with code in this repository.

## Related Projects

This crate is part of a family of Rust projects that share the same
coding standards, tooling, and workflows:

Cargo plugins:

- `cargo-fmt-toml` - Format and normalize Cargo.toml files
- `cargo-nightly` - Nightly toolchain management
- `cargo-plugin-utils` - Shared utilities for cargo plugins
- `cargo-propagate-features` - Propagate features to dependencies
- `cargo-version-info` - Dynamic version computation

Other Rust crates:

- `dotenvage` - Environment variable management

All projects use identical configurations for rustfmt, clippy,
markdownlint, cocogitto, and git hooks. When making changes to
tooling or workflow conventions, apply them consistently across
all repositories.

## Project Overview

`cargo-nightly` is a Cargo subcommand that forwards commands to the
Rust nightly toolchain. It solves the problem that cargo aliases
cannot use `+nightly` syntax directly. The tool enables aliases like:

```toml
[alias]
clippy-all = "nightly clippy --workspace --all-targets --all-features -- -D warnings"
```

Published to crates.io at https://crates.io/crates/cargo-nightly

## Build Commands

```bash
# Build
cargo build

# Run tests
cargo test
```

## Testing and Linting

```bash
# Format check (requires nightly)
sh scripts/fmt.sh --check

# Format code
sh scripts/fmt.sh

# Clippy (requires nightly)
cargo clippy --all-targets --all-features -- -D warnings -W missing-docs
```

## Code Style

- **Rust Edition**: 2024, MSRV 1.94.1
- **Formatting**: Uses nightly rustfmt with vertical imports grouped
  by std/external/crate (see `rustfmt.toml`)
- **Clippy**: Nightly with strict settings (max 120 lines/function,
  nesting threshold 5)
- **Disallowed variable names**: foo, bar, baz, qux, i, n
- **Documentation**: All public items must have docs (`-W missing-docs`)

## Architecture

The codebase is minimal by design:

- `src/main.rs` - Binary entry point. Forwards arguments to
  `cargo +nightly <args>`, inheriting stdin/stdout/stderr and exit code
- `build.rs` - Installs hooks only in the owning checkout.
- Executable identity is embedded Cargo package metadata.

## Version management

`CI` is the sole publisher. Its shared tested-revision pipeline owns
signed version preparation, package tests and six-target binaries.
See `.github/PUBLISH.md`. Never move release tags or add a publisher.

## Git workflow

- Commits follow Angular Conventional Commits:
  `<type>(<scope>): <subject>`
- Types: feat, fix, docs, refactor, test, style, perf, build, ci,
  chore, revert
- Use lowercase for type, scope, and subject start
- Never bypass git hooks with `--no-verify`
- Never execute `git push` - user must push manually
- Prefer `git rebase` over `git merge` for linear history

Git hooks in `.githooks/` are auto-installed via `rhusky` during
build.

## Markdown formatting

- Maximum line length: 70 characters
- Use `-` for unordered lists (not `*` or `+`)
- Use sentence case for headers (not Title Case)
- Indent nested lists with 2 spaces
- Surround lists and code blocks with blank lines

### Markdown linting

Configuration is in `.markdownlint.json`:

- Line length: 70 characters (MD013)
- Code blocks: unlimited line length

```bash
markdownlint '**/*.md' --ignore node_modules --ignore target
```
