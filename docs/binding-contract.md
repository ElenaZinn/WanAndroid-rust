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

The next implementation step is a generated native adapter that owns a Home-capable `CoreHandle`, creates the production HTTP client and exposes `dispatch_json` / `snapshot_json` to Kotlin. Auth, Search, Project and Collection will be added to the same opaque-handle pattern rather than passed directly to Kotlin.

```text
create(repository) -> opaque handle
snapshot(handle) -> JSON snapshot
dispatch(handle, JSON action) -> JSON snapshot
destroy(handle)
```

The handle synchronizes access internally. UI adapters convert snapshots to `StateFlow` and must not call Retrofit or assemble WanAndroid requests.
