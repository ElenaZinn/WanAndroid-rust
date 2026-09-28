package com.elena.wanandroidrust.rust

import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow

/**
 * Concrete JNI surface for `wanandroid_core` C ABI.
 * Native calls are synchronous in the current Rust runner, so callers must dispatch from a
 * background coroutine in production. This class owns the opaque handle and releases it.
 */
class JniRustCoreBinding(timeoutMs: Long = 15_000L) : NativeRustCoreBinding {
    private var handle: Long = nativeCreate(timeoutMs)
    private val _snapshotJson = MutableStateFlow(nativeSnapshot(handle))
    override val snapshotJson: StateFlow<String> = _snapshotJson

    override fun dispatchJson(actionJson: String) {
        check(handle != 0L) { "Rust core has been closed" }
        _snapshotJson.value = nativeDispatch(handle, actionJson)
    }

    override fun close() {
        if (handle != 0L) {
            nativeDestroy(handle)
            handle = 0L
        }
    }

    private external fun nativeCreate(timeoutMs: Long): Long
    private external fun nativeSnapshot(handle: Long): String
    private external fun nativeDispatch(handle: Long, actionJson: String): String
    private external fun nativeDestroy(handle: Long)

    companion object {
        init { System.loadLibrary("wanandroid_core_jni") }
    }
}
