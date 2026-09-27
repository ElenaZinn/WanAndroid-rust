# WanAndroid-rust

Rust-first WanAndroid client foundation. Android UI is Kotlin/Jetpack Compose; shared API contracts, domain models and business state transitions live in Rust.

## Current scope

- Rust `wanandroid-core` crate with API DTOs, domain models, repository abstraction and Home interactor.
- Explicit `Action -> Reducer -> State + Effect` data flow.
- Android Kotlin Compose shell with a platform-neutral `RustCoreGateway` seam.
- No iOS code in the current phase.

## Local verification

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

The Android module is intentionally a small Kotlin shell until the Rust Android binding is selected and generated. Kotlin UI must dispatch actions through the gateway and must not own API paths, cookies, pagination, or response mapping.

## Architecture

```text
Compose UI -> ViewModel/StateFlow -> RustCoreGateway -> Interactor
                                               |             |
                                               v             v
                                          Repository     State + Effect
                                               |
                                               v
                                      WanAndroid API
```

GitHub remote: `git@github.com:ElenaZinn/WanAndroid-rust.git`
