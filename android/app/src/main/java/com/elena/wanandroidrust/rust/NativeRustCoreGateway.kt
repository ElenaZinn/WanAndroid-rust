package com.elena.wanandroidrust.rust

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.stateIn

/**
 * Native-facing implementation seam. The generated JNI/UniFFI adapter should implement this
 * interface; Compose and ViewModel code must depend only on RustCoreGateway.
 */
interface NativeRustCoreBinding {
    val snapshotJson: StateFlow<String>
    fun dispatchJson(actionJson: String)
    fun close()
}

class NativeRustCoreGateway(
    private val binding: NativeRustCoreBinding,
    scope: CoroutineScope,
    snapshotDecoder: (String) -> HomeSnapshot,
) : RustCoreGateway {
    override val homeState: StateFlow<HomeSnapshot> = binding.snapshotJson
        .map(snapshotDecoder)
        .stateIn(
            scope = scope,
            started = SharingStarted.Eagerly,
            initialValue = snapshotDecoder(binding.snapshotJson.value),
        )

    override val authState: StateFlow<AuthSnapshot> =
        kotlinx.coroutines.flow.MutableStateFlow(AuthSnapshot())

    override fun dispatch(action: HomeAction) {
        binding.dispatchJson(action.toJson())
    }

    override fun dispatch(action: AuthAction) {
        binding.dispatchJson(action.toJson())
    }

    fun close() = binding.close()
}

private fun HomeAction.toJson(): String = when (this) {
    HomeAction.Load -> "{\"type\":\"home_load\"}"
    HomeAction.Refresh -> "{\"type\":\"home_refresh\"}"
    HomeAction.LoadNextPage -> "{\"type\":\"home_load_next_page\"}"
    is HomeAction.ToggleCollect -> "{\"type\":\"home_toggle_collect\",\"article_id\":$articleId}"
}

private fun AuthAction.toJson(): String = when (this) {
    AuthAction.Restore -> "{\"type\":\"auth_restore\"}"
    AuthAction.Logout -> "{\"type\":\"auth_logout\"}"
    is AuthAction.Login -> "{\"type\":\"auth_login\",\"username\":${username.jsonString()},\"password\":${password.jsonString()}}"
}

private fun String.jsonString(): String = buildString {
    append('"')
    for (character in this@jsonString) {
        when (character) {
            '\\' -> append("\\\\")
            '"' -> append("\\\"")
            '\n' -> append("\\n")
            '\r' -> append("\\r")
            '\t' -> append("\\t")
            else -> append(character)
        }
    }
    append('"')
}
