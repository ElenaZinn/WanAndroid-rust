# Rust Android native artifact layout

The Android CMake target expects Rust `cdylib` artifacts at:

```text
android/rust-lib/<abi>/libwanandroid_core.so
```

Expected ABIs are `arm64-v8a` and `x86_64` for the first debug/native integration. The artifacts are intentionally not checked in. A host with the Android Rust targets and NDK can produce them with the project-local Rust toolchain, for example:

```text
rustup target add aarch64-linux-android x86_64-linux-android
cargo build -p wanandroid-core --target aarch64-linux-android
cargo build -p wanandroid-core --target x86_64-linux-android
```

The CMake/JNI layer owns only string and opaque-handle marshalling. It does not implement HTTP, Cookie parsing, JSON DTO mapping, or pagination.
