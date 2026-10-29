# Contributing Guidelines

## Pre-Flight Verification

Before opening a Pull Request, verify both the Rust workspace and Go HTTP package:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
go vet ./...
go test -race ./...
```
