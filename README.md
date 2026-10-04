# WanAndroid-rust

Rust-first WanAndroid client foundation. Android UI is Kotlin/Jetpack Compose; shared API contracts, domain models and business state transitions live in Rust.

## Origin

This repository is the Rust-first rewrite of [ElenaZinn/WanAndroid](https://github.com/ElenaZinn/WanAndroid),
which stays the behavioural reference for the client. The migration moves API contracts, DTO mapping,
session/Cookie handling, pagination and business state transitions into Rust, and leaves Android as a
thin Compose layer that renders snapshots.

Porting history and the remaining matrix live in `docs/porting-guide.md`.

## Demo

A recorded walkthrough lives at [`docs/demo/wanandroid-rust-demo.mp4`](docs/demo/wanandroid-rust-demo.mp4):
home banner carousel, article detail, search, project categories, collection and account.

> The recording runs against a local proxy. `wanandroid.com`'s TLS certificate expired on
> 2026-10-03 and the site redirects all plain HTTP to HTTPS, so a client that validates
> certificates cannot reach the real API. The proxy relays real data; the app itself is unchanged.
> See `docs/demo-recording.md` for the setup.

## Current scope

- Rust `wanandroid-core` crate with API DTOs, domain models, Home/Auth/Search/Project/Collection repository abstractions and interactors.
- Explicit `Action -> Reducer -> State + Effect` data flow.
- Production `ReqwestHttpClient` with Rustls and timeout is now available behind the `HttpClient` trait.
- JSON-friendly binding contract in `docs/binding-contract.md`.
- Migration matrix and native binding rollout in `docs/porting-guide.md`.
- UniFFI-generated Android bridge: `rust/wanandroid-core/src/wanandroid.udl`, generated Kotlin bindings, and `UniFfiRustCoreBinding.kt`.
- Complete Compose UI: banner carousel with auto-advance, article cards, pull-to-refresh, infinite scroll, hot-key search, project category tabs, WebView detail and icon navigation.
- No iOS code in the current phase.

## Local verification

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

`cargo` is installed for this workspace under `.tools/`; it is ignored by Git. Do not build an APK as part of the default validation because native/Gradle builds are intentionally expensive; Rust checks remain the fast gate.

`cargo fmt`, `cargo clippy` and `cargo test` only require the Rust toolchain, so the default validation needs no Android SDK. Assembling an APK is a separate, deliberate step; the toolchain layout and build requirements are described in `docs/demo-recording.md`.

## Technical depth catch-up checklist

See `七问见技术深度补课清单.md` for the full rationale behind the architecture. The seven priority-0 questions are:

1. **Why do the interactor and state transitions live in Rust?** Rust is the single business source of truth for API semantics, DTO mapping, session/Cookie handling, pagination and state transitions. This prevents Android and future iOS clients from implementing subtly different rules.
2. **Why migrate from hand-written C ABI/JNI to UniFFI?** The former bridge required manually maintaining opaque handles, JNI signatures, memory ownership and platform glue. UniFFI generates the language bindings from one UDL boundary, reducing synchronization risk while keeping the same Rust-owned business core and leaving room for future Swift bindings.
3. **Why choose `Action -> Reducer -> State + Effect`?** Actions make intent explicit, reducers keep state transitions deterministic and testable, and effects isolate I/O. The split avoids putting blocking work in UI code without hiding the state machine inside platform-specific ViewModels.
4. **What does the `HttpClient` trait abstraction solve?** Interactors and repositories depend on a small transport contract rather than `reqwest`. Tests can use deterministic fakes, while production uses the Rustls-backed client with timeout, headers and Cookie support.
5. **How is the boundary defined so Kotlin does not construct requests?** Kotlin sends typed feature intent through the gateway; Rust owns endpoint URLs, query/form encoding, DTO parsing, pagination, authentication, Cookie/session behavior and error mapping. Kotlin only observes snapshots and renders Compose UI.
6. **Why does CI run `clippy -D warnings`?** Warnings are treated as architecture and correctness feedback rather than deferred cleanup. Failing on warnings keeps the shared Rust boundary reviewable and prevents platform integration from masking problems in the core crate.
7. **What is the NDK + Gradle Rust `cdylib` packaging flow?** The Rust script cross-compiles `libuniffi_wanandroid.so` for `arm64-v8a` and `x86_64` with the pinned NDK targets; Gradle packages those files through `jniLibs`, and UniFFI's generated Kotlin/JNA runtime loads them. This is an optional native packaging path, not part of the fast default Rust validation.


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

## Repositories

- This rewrite: `git@github.com:ElenaZinn/WanAndroid-rust.git`
- Original client (behavioural reference): <https://github.com/ElenaZinn/WanAndroid>
