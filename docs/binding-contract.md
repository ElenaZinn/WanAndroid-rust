# Rust binding contract

The current binding contract is JSON-friendly and platform-neutral. It is intentionally independent of JNI, UniFFI, and Android lifecycle classes.

## Actions

`CoreAction` is a tagged enum. A binding can accept serialized actions or map native calls to the same Rust enum:

- `home_load`
- `home_refresh`
- `home_load_next_page`
- `home_toggle_collect`
- `auth_restore`
- `auth_login`
- `auth_logout`

## Snapshot

`CoreSnapshot` contains feature snapshots. `HomeSnapshot` exposes plain arrays and nullable error text; it never exposes `LoadState` implementation details to Kotlin.

The binding has been switched from hand-written JNI/C ABI calls to UniFFI. The Rust UDL is `rust/wanandroid-core/src/wanandroid.udl`; generated Kotlin is placed under `android/app/src/main/java/uniffi/wanandroid/`; `UniFfiRustCoreBinding` wraps the generated `CoreHandle`.

The Android Gradle module packages generated Rust `.so` files from `android/rust-lib/` as `jniLibs`; it no longer needs the hand-written CMake/JNI bridge.

The same Rust `CoreHandle` and UDL can later generate Swift bindings for iOS. This change deliberately adds no iOS application code.

```text
create(repository) -> opaque handle
snapshot(handle) -> JSON snapshot
dispatch(handle, JSON action) -> JSON snapshot
destroy(handle)
```

The handle synchronizes access internally. UI adapters convert snapshots to `StateFlow` and must not call Retrofit or assemble WanAndroid requests.
