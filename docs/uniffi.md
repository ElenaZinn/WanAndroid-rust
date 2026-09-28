# UniFFI binding

The shared Rust API is defined in `rust/wanandroid-core/src/wanandroid.udl` and implemented by the Rust `CoreHandle` object.

## Android

Generate Kotlin bindings with:

```text
./scripts/generate-uniffi-kotlin.sh
```

The generated file is placed under:

```text
android/app/src/main/java/uniffi/wanandroid/wanandroid.kt
```

`UniFfiRustCoreBinding` owns the generated `CoreHandle`; `NativeRustCoreGateway` remains the application-facing adapter used by Compose. JNA is used by UniFFI's generated Kotlin runtime to load the Rust shared library.

## Future iOS

The same UDL and Rust `CoreHandle` can generate Swift bindings later. No iOS application code is included in the current repository phase. The binding contract intentionally exposes only JSON action/snapshot methods, so iOS can reuse the same business state semantics without duplicating API, Cookie, pagination, or DTO logic.
