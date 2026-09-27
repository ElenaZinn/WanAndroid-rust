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

The next implementation step is an opaque core handle that owns repository/interactors and exposes:

```text
create() -> handle
snapshot(handle) -> JSON snapshot
 dispatch(handle, JSON action) -> JSON effects/snapshot
 destroy(handle)
```

The handle must be single-owner or explicitly synchronized. UI adapters must convert snapshots to `StateFlow` and must not call Retrofit or assemble WanAndroid requests.
