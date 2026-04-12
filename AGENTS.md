# AGENTS.md

## Cursor Cloud specific instructions

### Overview

Koe is a **native macOS voice input tool** built with Rust (core logic), Objective-C (macOS shell/UI), and Swift (MLX & Apple Speech bridges). The full app (`make build` / `make run`) requires macOS + Xcode + xcodegen. On Linux (Cloud Agent VMs), only the **Rust workspace** can be built and tested.

### What works on Linux

| Task | Command |
|---|---|
| Format check | `cargo fmt --check` |
| Clippy lint | `cargo clippy --no-default-features -p koe-asr -p koe-core -p koe-cli` |
| Type check | `cargo check --no-default-features -p koe-asr -p koe-core -p koe-cli` |
| Unit tests | `cargo test --no-default-features -p koe-asr -p koe-core -p koe-cli` |
| Build CLI | `cargo build --release -p koe-cli` |
| Run CLI | `./target/release/koe --help` / `./target/release/koe model list` |

### Key caveats

- **Always pass `--no-default-features`** when checking/testing `koe-core` or the whole workspace. The default features (`mlx`, `apple-speech`, `sherpa-onnx`) are macOS-specific; `sherpa-onnx` builds on Linux but `mlx` and `apple-speech` are compile-time markers for macOS FFI code.
- **`koe-cli`** uses `default-features = false` for `koe-core` already, so it builds without extra flags.
- **`sherpa-onnx` feature** can be tested on Linux: `cargo check --features sherpa-onnx -p koe-core`.
- **`libopus-dev`** must be installed (`apt-get install -y libopus-dev`) for the `audiopus` crate to compile.
- The Rust workspace requires **Rust >= 1.85** (specified in `Cargo.toml`).
- The Xcode/Objective-C/Swift parts (`KoeApp/`, `KoeMLX/`, `KoeAppleSpeech/`) cannot be compiled on Linux.
- There is no `cargo test` target that requires ASR/LLM credentials; the existing `koe-asr/tests/api_test.rs` is an integration test requiring real API keys and is not run in CI.
- Clippy warnings are expected (configured as `warn` in workspace `Cargo.toml`); they do not fail the build.
