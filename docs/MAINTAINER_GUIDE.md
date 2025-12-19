# Maintainer Guide - Meoslabs llama-cpp-rs

This repository is a private fork of [utilityai/llama-cpp-rs](https://github.com/utilityai/llama-cpp-rs), maintained by Meoslabs for the MEOS ecosystem.

## 🎯 Strategic Goals

1.  **Stability**: We prioritize stability on mobile (Android/iOS) and Linux platforms over cutting-edge features we don't use.
2.  **Multimodal**: We actively support and fix build issues related to `llama.cpp`'s multimodal (MTMD) capabilities.
3.  **Hardware Acceleration**: We treat OpenCL (Android) and Metal (iOS) as first-class citizens.

## 🛠️ The `xtask` Commander

We use `cargo xtask` to automate maintenance. Run it from the repo root:

```bash
# List available commands
cargo run -p xtask -- --help

# Verify repository state (submodules, patches)
cargo run -p xtask -- verify
```

## 🔄 Upstream Synchronization Protocol

To sync with upstream `utilityai`:

1.  **Add Remote**: Ensure you have the upstream remote:
    ```bash
    git remote add upstream https://github.com/utilityai/llama-cpp-rs.git
    ```
2.  **Fetch & Merge**:
    ```bash
    git fetch upstream
    git merge upstream/main
    ```
3.  **Resolve Conflicts**: Pay close attention to `llama-cpp-sys-2/build.rs` and `Cargo.toml`. Our feature flags (`opencl`, `mtmd`) must be preserved.
4.  **Verify Patches**: Use `cargo xtask verify` to check if our essential patches to `CMakeLists.txt` or `build.rs` survived the merge.

## 🧱 The "Mobile Fortress" (Build Verification)

Before pushing, verify builds for our target platforms.

### Android
Requires `cargo-ndk` and proper NDK environment variables.

```bash
# Check build
cargo ndk -t arm64-v8a -p 28 build -p llama-cpp-2 --features opencl,mtmd
```

### iOS
Requires macOS.

```bash
cargo build --target aarch64-apple-ios --features metal,mtmd
```

## ⚠️ Known Patches

We maintain specific patches that differ from upstream. **Do not lose these during merges.**

### 1. MTMD Build Support (`llama-cpp-sys-2/build.rs`)
We explicitly add `tools/mtmd` to the include paths in `build.rs`. Upstream often misses this because MTMD is experimental.

### 2. Server Dependency (`tools/CMakeLists.txt`)
We force `add_subdirectory(server)` when building `mtmd` tools because `llama-cli` depends on `server-context` in newer `llama.cpp` versions.

### 3. OpenCL on Android
We enable `opencl` feature by default for Android in our consuming apps, so `llama-cpp-sys-2` must expose it correctly.

