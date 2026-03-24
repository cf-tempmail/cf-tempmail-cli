# CF-TEMPMAIL CLI KNOWLEDGE BASE

**Generated:** 2026-03-25
**Commit:** d5e541d
**Branch:** main

## OVERVIEW

Rust CLI for temporary email service. Talks to Cloudflare Worker backend via REST API. 3-file flat structure, ~340 LOC.

## STRUCTURE

```
cli/
├── src/main.rs    # CLI entry, clap commands, tokio runtime
├── src/api.rs     # HTTP client (reqwest + rustls)
├── src/config.rs  # TOML config persistence (~/.config/cf-tempmail/)
└── .github/workflows/release.yml  # Multi-platform CI
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| Add CLI command | src/main.rs | Add to `Commands` enum + match arm |
| Modify API calls | src/api.rs | `Client` struct, async methods |
| Change config format | src/config.rs | `Config`/`Session` structs |
| Add dependency | Cargo.toml | Feature flags pattern: `{ version = "X", features = ["..."], default-features = false }` |
| CI/release changes | .github/workflows/release.yml | Tag-triggered (`v*`) |
| Default server URL | src/config.rs:37 | `https://temp-email.mrwuliu.top` |

## CONVENTIONS

**Dependency Pattern** (explicit features, avoid defaults):
```toml
reqwest = { version = "0.12", features = ["json", "rustls-tls"], default-features = false }
```

**Error Handling**: Use `anyhow::Result` everywhere. No custom error types.

**CLI Structure**: clap derive macros (`Parser`, `Subcommand`)

**Serde**: `#[serde(rename = "camelCase")]` for JSON API compatibility

**Async**: `#[tokio::main]` on main, tokio with `full` features

**Config Storage**: Platform-specific via `dirs` crate, TOML format

**Dead Code**: `#[allow(dead_code)]` on struct fields needed for deserialization but unused

## ANTI-PATTERNS (THIS PROJECT)

**Missing Infrastructure**:
- No tests (`#[test]`, tests/ directory, cargo test)
- No lint CI (`cargo clippy`, `cargo fmt --check`)
- No HTTP timeout on reqwest client
- No retry logic for API calls
- No graceful shutdown for `listen` command (SIGINT/SIGTERM)

**Avoid**:
- OpenSSL — use `rustls-tls` (already configured)
- `unwrap()` in production code — use `?` or `unwrap_or_*` patterns

## UNIQUE STYLES

- Binary name differs from package name: `cf-tempmail` (binary) vs `cf-tempmail-cli` (package)
- Release artifacts: `cf-tempmail-{arch}-{os}` naming
- Bilingual README (Chinese primary)

## COMMANDS

```bash
# Development
cargo run -- new --prefix test     # Run CLI with args
cargo build --release              # Build binary → target/release/cf-tempmail

# Cross-compile (requires gcc-aarch64-linux-gnu)
cargo build --release --target aarch64-unknown-linux-gnu

# Debian package (requires cargo install cargo-deb)
cargo deb --target x86_64-unknown-linux-gnu

# Release (trigger CI)
git tag v0.1.0 && git push --tags
```

## NOTES

- **Monorepo**: `/cf-tempmail/cli` (this) + `/cf-tempmail/worker` (Cloudflare Worker backend)
- **No workspace**: Each subproject is independent
- **Default URL**: `https://temp-email.mrwuliu.top` — configure via `cf-tempmail config --baseurl <url>`
- **Config location**: `~/.config/cf-tempmail/config.toml` (Linux), `~/Library/Application Support/cf-tempmail/` (macOS)
