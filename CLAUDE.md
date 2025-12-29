# file-based-semaphore Development Guide

This document provides context for Claude Code when working on file-based-semaphore.

## Tool Overview

**Purpose:** Cross-platform file-based semaphore for process coordination

**Short Name:** `sema`
**Long Name:** `file-semaphore`

**Language:** Rust
**Version:** 0.1.0
**Tests:** 95 passing
**Dependencies:** 0 (zero runtime dependencies)

## Architecture

**Library:** `src/lib.rs` - Core semaphore implementation
**CLI:** `src/main.rs` - Command-line interface
**Tests:** `tests/` - Integration tests

## Key Principles

1. **Zero runtime dependencies** - Uses only Rust standard library
2. **Cross-platform** - Works on Linux, macOS, Windows
3. **Stale lock detection** - Handles process crashes gracefully
4. **Atomic operations** - Uses exclusive file creation for locking

## Development Workflow

### Building
```bash
cargo build
cargo build --release
```

### Testing
```bash
cargo test
cargo test --all-features
```

### Linting
```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

### Installing Locally
```bash
cargo install --path .
sema --help  # Short name (recommended)
file-semaphore --help  # Long name
```

## Code Standards

- **Formatting:** Run `cargo fmt` before committing
- **Linting:** Zero clippy warnings (`-D warnings`)
- **Testing:** Maintain 80%+ coverage
- **Documentation:** All public items documented with `///`
- **Error handling:** Use `?` operator, avoid `unwrap()` in production

## Common Tasks

### Adding a New Feature
1. Implement in `src/lib.rs` or `src/main.rs`
2. Add tests in `tests/` or `src/lib.rs #[cfg(test)]`
3. Update README.md with examples
4. Run `cargo test && cargo clippy && cargo fmt`

### Debugging
```bash
# Run with verbose output
cargo run -- acquire /tmp/test.lock --timeout 5 --verbose

# Check lock file contents
cat /tmp/test.lock
```

### Release Process
1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Run all tests: `cargo test --all-features`
4. Tag release: `git tag vX.Y.Z`
5. Push: `git push origin vX.Y.Z`

## Repository Links

- **GitHub:** https://github.com/tuulbelt/file-based-semaphore
- **Homepage:** https://tuulbelt.github.io/tuulbelt/tools/file-based-semaphore/
- **Meta Repo:** https://github.com/tuulbelt/tuulbelt
- **Issues:** https://github.com/tuulbelt/tuulbelt/issues (use label: `file-based-semaphore`)

## Project Structure

```
file-based-semaphore/
├── src/
│   ├── lib.rs           # Core library (Semaphore, SemaphoreGuard)
│   └── main.rs          # CLI implementation
├── tests/
│   ├── integration.rs   # Integration tests
│   └── concurrent.rs    # Concurrency tests
├── examples/            # Example usage
├── scripts/
│   └── dogfood-*.sh     # Dogfooding scripts
├── Cargo.toml
├── README.md
├── SPEC.md              # Lock file format specification
└── CLAUDE.md            # This file
```

## Security Considerations

- **Path traversal:** Validate lock file paths
- **Stale locks:** Detect and handle gracefully
- **Race conditions:** Use atomic file operations
- **Permissions:** Lock files created with 0600 permissions

## Dogfooding

This tool is used by other Tuulbelt tools:
- **test-port-resolver** - Uses semaphore for port allocation locking

See `DOGFOODING_STRATEGY.md` for validation scripts.

## Known Issues

See `docs/KNOWN_ISSUES.md` in the meta repo.

## Getting Help

- Check README.md for usage examples
- Check SPEC.md for lock file format
- See examples/ directory for code samples
- Open issues at: https://github.com/tuulbelt/tuulbelt/issues

---

**Last Updated:** 2025-12-29
**Part of:** [Tuulbelt](https://github.com/tuulbelt/tuulbelt)
