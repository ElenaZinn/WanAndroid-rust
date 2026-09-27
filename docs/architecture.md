# Architecture

## State flow

Each feature receives an `Action`. The reducer applies synchronous state changes and emits an `Effect` when I/O is required. The effect runner calls a repository and feeds its result back as an internal action.

```text
User event
   -> Action
   -> reduce(State, Action)
   -> State snapshot + Effect
   -> Repository/HTTP
   -> internal Action
```

The Rust core owns API paths, DTO decoding, error mapping, pagination and session rules. Kotlin owns lifecycle, Compose rendering, navigation, WebView/external links and Android-specific persistence adapters.

## Android boundary

`RustCoreGateway` is the temporary Kotlin seam for the generated Rust binding. It exposes snapshots and action dispatch without allowing UI code to call Retrofit. The next binding step can replace its implementation with JNI, UniFFI or a C ABI while keeping Compose screens unchanged.

The Rust transport boundary accepts a structured `HttpRequest` containing a relative API path and headers. Repositories build these requests, load session cookies from `SessionStore`, and attach the `Cookie` header before the platform adapter executes the request. Kotlin UI and platform adapters therefore do not own session or request-construction rules.

## Infrastructure diagram

```text
+------------------+       +---------------------------+
| WanAndroid API   | HTTPS | Rust wanandroid-core       |
| www.wanandroid   |<----->| DTO -> repository ->       |
+------------------+       | interactor -> state/effect |
                           +-------------+-------------+
                                         |
                             binding / gateway boundary
                                         |
                           +-------------v-------------+
                           | Kotlin Android application |
                           | ViewModel + StateFlow       |
                           | Jetpack Compose UI          |
                           +-----------------------------+
```
