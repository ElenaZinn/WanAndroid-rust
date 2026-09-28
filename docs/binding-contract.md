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

The current native path is now concrete rather than a Preview gateway: Rust exports an opaque C ABI handle, `JniRustCoreBinding` calls it, and `MainActivity` constructs `NativeRustCoreGateway`. Effects are dispatched on `Dispatchers.IO`; Compose observes decoded snapshots only.

```text
create(repository) -> opaque handle
snapshot(handle) -> JSON snapshot
dispatch(handle, JSON action) -> JSON snapshot
destroy(handle)
```

The handle synchronizes access internally. UI adapters convert snapshots to `StateFlow` and must not call Retrofit or assemble WanAndroid requests.
