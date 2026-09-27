# Android binding boundary

Rust is built as an `rlib`, `cdylib`, and `staticlib` so a later Android adapter can select JNI, UniFFI, or a C ABI without changing repositories or interactors.

The binding boundary is:

- Rust: `CoreHandle`, `CoreAction`, `CoreSnapshot`, JSON serialization and lifecycle ownership.
- Native adapter: owns one `CoreHandle`, marshals `dispatch_json`/`snapshot_json`, and publishes snapshots to Kotlin.
- Kotlin: `NativeRustCoreBinding` and `NativeRustCoreGateway` only translate UI actions and render decoded snapshots.

No Android build is committed yet because the repository does not currently contain the generated binding toolchain or a Gradle wrapper. Adding a generated binding should be a separate, reproducible build step rather than hand-written API/network logic in the Compose module.
