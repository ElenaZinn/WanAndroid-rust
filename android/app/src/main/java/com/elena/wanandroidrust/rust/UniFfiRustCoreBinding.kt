package com.elena.wanandroidrust.rust

import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import uniffi.wanandroid.CoreHandle

/**
 * UniFFI-generated Kotlin binding wrapper. The same Rust `CoreHandle` API can generate Swift
 * bindings later; this repository deliberately contains no iOS source code.
 */
class UniFfiRustCoreBinding(timeoutMs: Long = 15_000L) : NativeRustCoreBinding {
    private val handle = CoreHandle(timeoutMs.toULong())
    private val _snapshotJson = MutableStateFlow(handle.snapshotJson())
    override val snapshotJson: StateFlow<String> = _snapshotJson

    override fun dispatchJson(actionJson: String) {
        _snapshotJson.value = handle.dispatchJson(actionJson)
    }

    override fun close() = handle.close()
}
