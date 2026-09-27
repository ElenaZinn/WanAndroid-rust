# Porting guide

## Current migration matrix

| Existing capability | Rust core | Android Compose surface | Status |
|---|---|---|---|
| Home banners/articles | `HomeRepository` + `HomeInteractor` | `HomeViewModel` gateway seam | Core implemented; native binding pending |
| Login/session/logout | `AuthRepository` + `AuthInteractor` + `SessionStore` | `AuthAction` gateway seam | Core implemented; native binding pending |
| Search/hot keys/author | `SearchRepository` + `SearchInteractor` | Contract ready; screen wiring next | Core implemented |
| Project categories/articles | `ProjectRepository` + `ProjectInteractor` | Contract ready; screen wiring next | Core implemented |
| Collection list/collect/uncollect | `CollectionRepository` + `CollectionInteractor` | Contract ready; screen wiring next | Core implemented |
| Native binding | `CoreHandle` JSON facade | `NativeRustCoreBinding` / `NativeRustCoreGateway` | Seam implemented; generated adapter pending |
| iOS | Not in current scope | Not in current scope | Deliberately deferred |

## Binding rollout

1. Provision a Rust/Android toolchain and choose JNI, UniFFI, or C ABI for the production adapter.
2. Implement a platform `HttpClient` and `SessionStore`; the Rust core must receive them through repository construction.
3. Make the native adapter own one `CoreHandle`, publish `snapshot_json` as a Kotlin `StateFlow`, and forward serialized actions.
4. Replace `PreviewRustCoreGateway` in `MainActivity` with the generated adapter.
5. Add an Android smoke test for startup, Home load, login error, and session restore.

The `NativeRustCoreGateway` file deliberately contains only action serialization and snapshot-flow adaptation. It must not gain API paths, Cookie parsing, pagination, or response DTOs.
