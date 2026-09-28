# Android binding boundary

Rust is built as an `rlib`, `cdylib`, and `staticlib` so a later Android adapter can select JNI, UniFFI, or a C ABI without changing repositories or interactors.

The binding boundary is:

- Rust: `CoreHandle`, `CoreAction`, `CoreSnapshot`, JSON serialization and lifecycle ownership.
- Native adapter: owns one `CoreHandle`, marshals `dispatch_json`/`snapshot_json`, and publishes snapshots to Kotlin.
- Kotlin: `NativeRustCoreBinding` and `NativeRustCoreGateway` only translate UI actions and render decoded snapshots.

The Android project now contains a Gradle 8.7 wrapper. Its normal distribution URL is remote, while this workspace can use the downloaded distribution through `GRADLE_USER_HOME` when available.

`scripts/build-rust-android.sh` builds the Rust `cdylib` for `arm64-v8a` and `x86_64` using Android NDK 27.2. The generated `.so` files are local build products and are intentionally ignored by Git. No APK build is required for the Rust/native source check.
