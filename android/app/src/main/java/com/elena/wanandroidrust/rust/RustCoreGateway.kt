package com.elena.wanandroidrust.rust

import kotlinx.coroutines.flow.StateFlow

data class HomeSnapshot(
    val bannerTitle: String? = null,
    val articles: List<ArticleSnapshot> = emptyList(),
    val isLoading: Boolean = false,
    val errorMessage: String? = null,
    val canLoadMore: Boolean = true,
)

data class ArticleSnapshot(
    val id: Long,
    val title: String,
    val author: String,
    val date: String,
    val collected: Boolean,
)

sealed interface HomeAction {
    data object Load : HomeAction
    data object Refresh : HomeAction
    data object LoadNextPage : HomeAction
    data class ToggleCollect(val articleId: Long) : HomeAction
}

/**
 * Kotlin UI's only gateway to the shared business core.
 *
 * Its production implementation will be generated from Rust (JNI/UniFFI/C ABI).
 * Kotlin must not add Retrofit calls, cookie handling, API URL paths or pagination
 * rules behind this interface.
 */
interface RustCoreGateway {
    val homeState: StateFlow<HomeSnapshot>
    fun dispatch(action: HomeAction)
}
