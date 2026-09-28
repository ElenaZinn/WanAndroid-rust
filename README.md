# WanAndroid-rust

Rust-first WanAndroid client foundation. Android UI is Kotlin/Jetpack Compose; shared API contracts, domain models and business state transitions live in Rust.

## Current scope

- Rust `wanandroid-core` crate with API DTOs, domain models, Home/Auth/Search/Project/Collection repository abstractions and interactors.
- Explicit `Action -> Reducer -> State + Effect` data flow.
- Production `ReqwestHttpClient` with Rustls and timeout is now available behind the `HttpClient` trait.
- JSON-friendly binding contract in `docs/binding-contract.md`.
- Migration matrix and native binding rollout in `docs/porting-guide.md`.
- UniFFI-generated Android bridge: `rust/wanandroid-core/src/wanandroid.udl`, generated Kotlin bindings, and `UniFfiRustCoreBinding.kt`.
- No iOS code in the current phase.

## Local verification

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

`cargo` is installed for this workspace under `.tools/`; it is ignored by Git. Do not build an APK as part of the default validation because native/Gradle builds are intentionally expensive; Rust checks remain the fast gate.
The current native path is concrete rather than a Preview gateway: Rust exports an opaque C ABI handle, `JniRustCoreBinding` calls it, and `MainActivity` constructs `NativeRustCoreGateway`. Effects are dispatched on `Dispatchers.IO`; Compose observes decoded snapshots only.

The five Compose feature surfaces are Home, Search, Project, Account/Login and Collection. Their ViewModel forwards typed actions to the gateway; Kotlin does not construct API requests or implement pagination.

The project now includes Gradle Wrapper 8.7 and an NDK 27.2 Rust artifact script. The local validation intentionally stops at Gradle configuration and Rust/native artifact generation; it does not assemble an APK.

## UniFFI

The Rust-first binding now uses UniFFI rather than the former hand-written C ABI/JNI bridge. The shared UDL is `rust/wanandroid-core/src/wanandroid.udl`; regenerate Android bindings with `./scripts/generate-uniffi-kotlin.sh`. Android packages the Rust cdylib as `libuniffi_wanandroid.so` and uses the generated Kotlin/JNA runtime. The same UDL can later generate Swift bindings; no iOS UI or application code is included.


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
