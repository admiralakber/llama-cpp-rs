# Maintainer Guide - Meoslabs llama-cpp-rs

This repository is a private fork of [utilityai/llama-cpp-rs](https://github.com/utilityai/llama-cpp-rs), maintained by Meoslabs for the MEOS ecosystem.

## 🎯 Strategic Goals

1.  **Stability**: We prioritize stability on mobile (Android/iOS) and Linux platforms. CPU and Metal (iOS/macOS) are our production baselines.
2.  **Multimodal**: We actively support and fix build issues related to `llama.cpp`'s multimodal (MTMD) capabilities.
3.  **Upstream Parity**: We follow `ggml-org/llama.cpp` closely. Experimental accelerators (Vulkan/OpenCL) are supported where possible but should not block updates.

## 🛠️ The `xtask` Commander

We use `cargo xtask` to automate maintenance. Run it from the repo root:

```bash
# List available commands
cargo run -p xtask -- --help

# Verify repository state (submodules, patches)
cargo run -p xtask -- verify
```

## 🔄 Upstream Synchronization Protocol (Drift Defender)

We use the "Drift Defender" protocol to stay in sync with `utilityai`. This is automated via `xtask`.

```bash
# Run the Drift Defender
cargo run -p xtask -- sync
```

This command will:
1.  Fetch `upstream/main`.
2.  Create a new branch (e.g., `chore/sync-upstream-1734685200`).
3.  Attempt a merge.
4.  Update submodules.
5.  **Verify** that our critical patches are still present.

### Manual Conflict Resolution
If `xtask sync` encounters merge conflicts:
1.  It will stop and ask you to resolve them.
2.  **CRITICAL**: Ensure `llama-cpp-sys-2/build.rs` still contains the `mtmd` include logic.
3.  **CRITICAL**: Ensure `llama-cpp-sys-2/llama.cpp/tools/CMakeLists.txt` includes `server` when `LLAMA_BUILD_SERVER` is on.
4.  After resolving, run `cargo xtask verify` manually.
5.  Commit and push the branch.

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

### 3. Accelerator Support (Vulkan/OpenCL)
We expose these features in `llama-cpp-sys-2`, but they are considered experimental/optional compared to the rock-solid CPU/Metal baseline.

