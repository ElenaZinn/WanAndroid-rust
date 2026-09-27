# Local development notes

## Rust

Install Rust through rustup, then run from the repository root:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

As checked on 2026-09-28 (GMT+8), this execution environment has no `cargo` binary, so Rust formatting, Clippy, and tests could not run. `gradle` and `android/gradlew` are also absent, and `ANDROID_HOME` is unset; Android Gradle checks are blocked until a local Gradle/JDK/SDK toolchain is provisioned. The Git remote is configured, but toolchain availability is independent of remote configuration.

## Android

Open `android/` in Android Studio. The Compose shell is intentionally independent of a generated Rust binding. `RustCoreGateway` is the seam that will later be implemented by JNI/UniFFI/C ABI.

The current Rust transport seam is covered by repository contract tests for relative paths, empty headers, and session-cookie forwarding. The Android module still has no Gradle wrapper, so Android checks require opening `android/` in Android Studio or provisioning a local Gradle/JDK/SDK toolchain.
