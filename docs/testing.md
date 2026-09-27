# Local development notes

## Rust

Install Rust through rustup, then run from the repository root:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

The current execution environment does not have `cargo` installed, so these commands have not yet been run here. No remote repository was configured.

## Android

Open `android/` in Android Studio. The Compose shell is intentionally independent of a generated Rust binding. `RustCoreGateway` is the seam that will later be implemented by JNI/UniFFI/C ABI.

The Android Gradle wrapper has not been copied yet; Android Studio can generate or sync it after selecting a local Gradle/JDK configuration.
