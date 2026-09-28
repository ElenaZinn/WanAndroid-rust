package com.elena.wanandroidrust.rust

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch

// Snapshot types mirror the JSON contract, not Rust implementation types.
data class HomeSnapshot(
    val banners: List<BannerSnapshot> = emptyList(),
    val articles: List<ArticleSnapshot> = emptyList(),
    val isLoading: Boolean = false,
    val errorMessage: String? = null,
    val canLoadMore: Boolean = true,
)

data class AuthSnapshot(
    val username: String? = null,
    val isLoading: Boolean = false,
    val errorMessage: String? = null,
)

data class SearchSnapshot(
    val hotKeys: List<HotKeySnapshot> = emptyList(),
    val articles: List<ArticleSnapshot> = emptyList(),
    val keyword: String = "",
    val author: String? = null,
    val isLoading: Boolean = false,
    val errorMessage: String? = null,
    val canLoadMore: Boolean = false,
)

data class ProjectSnapshot(
    val categories: List<ProjectCategorySnapshot> = emptyList(),
    val articles: List<ProjectArticleSnapshot> = emptyList(),
    val selectedCategory: Long? = null,
    val isLoading: Boolean = false,
    val errorMessage: String? = null,
    val canLoadMore: Boolean = false,
)

data class CollectionSnapshot(
    val articles: List<ArticleSnapshot> = emptyList(),
    val isLoading: Boolean = false,
    val errorMessage: String? = null,
    val canLoadMore: Boolean = true,
    val pendingChange: Boolean = false,
)

data class BannerSnapshot(val id: Long, val title: String, val imagePath: String, val url: String)
data class ArticleSnapshot(
    val id: Long,
    val title: String,
    val link: String,
    val author: String,
    val date: String,
    val collected: Boolean,
    val chapterName: String,
)
data class HotKeySnapshot(val id: Long, val name: String, val link: String)
data class ProjectCategorySnapshot(val id: Long, val name: String, val order: Int, val visible: Int)
data class ProjectArticleSnapshot(
    val id: Long,
    val title: String,
    val link: String,
    val author: String,
    val date: String,
    val collected: Boolean,
    val chapterName: String,
    val envelopePic: String,
    val thumbnail: String,
)

sealed interface HomeAction {
    data object Load : HomeAction
    data object Refresh : HomeAction
    data object LoadNextPage : HomeAction
    data class ToggleCollect(val articleId: Long) : HomeAction
}

sealed interface AuthAction {
    data object Restore : AuthAction
    data class Login(val username: String, val password: String) : AuthAction
    data object Logout : AuthAction
}

sealed interface SearchAction {
    data object LoadHotKeys : SearchAction
    data class Submit(val keyword: String) : SearchAction
    data class Author(val author: String) : SearchAction
    data object LoadNextPage : SearchAction
}

sealed interface ProjectAction {
    data object LoadCategories : ProjectAction
    data class SelectCategory(val id: Long) : ProjectAction
    data object LoadNextPage : ProjectAction
}

sealed interface CollectionAction {
    data object Load : CollectionAction
    data object Refresh : CollectionAction
    data object LoadNextPage : CollectionAction
    data class Collect(val articleId: Long) : CollectionAction
    data class Uncollect(val originId: Long) : CollectionAction
}

interface RustCoreGateway {
    val homeState: StateFlow<HomeSnapshot>
    val authState: StateFlow<AuthSnapshot>
    val searchState: StateFlow<SearchSnapshot>
    val projectState: StateFlow<ProjectSnapshot>
    val collectionState: StateFlow<CollectionSnapshot>
    fun dispatch(action: HomeAction)
    fun dispatch(action: AuthAction)
    fun dispatch(action: SearchAction)
    fun dispatch(action: ProjectAction)
    fun dispatch(action: CollectionAction)
}

interface NativeRustCoreBinding {
    val snapshotJson: StateFlow<String>
    fun dispatchJson(actionJson: String)
    fun close()
}

class NativeRustCoreGateway(
    private val binding: NativeRustCoreBinding,
    scope: CoroutineScope,
    snapshotDecoder: (String) -> CoreSnapshot,
) : RustCoreGateway {
    private val snapshots: StateFlow<CoreSnapshot> = binding.snapshotJson
        .map(snapshotDecoder)
        .stateIn(scope, SharingStarted.Eagerly, snapshotDecoder(binding.snapshotJson.value))

    override val homeState = snapshots.map { it.home }.stateIn(scope, SharingStarted.Eagerly, snapshots.value.home)
    override val authState = snapshots.map { it.auth }.stateIn(scope, SharingStarted.Eagerly, snapshots.value.auth)
    override val searchState = snapshots.map { it.search }.stateIn(scope, SharingStarted.Eagerly, snapshots.value.search)
    override val projectState = snapshots.map { it.project }.stateIn(scope, SharingStarted.Eagerly, snapshots.value.project)
    override val collectionState = snapshots.map { it.collection }.stateIn(scope, SharingStarted.Eagerly, snapshots.value.collection)

    override fun dispatch(action: HomeAction) = scope.launch(Dispatchers.IO) { binding.dispatchJson(action.toJson()) }
    override fun dispatch(action: AuthAction) = scope.launch(Dispatchers.IO) { binding.dispatchJson(action.toJson()) }
    override fun dispatch(action: SearchAction) = scope.launch(Dispatchers.IO) { binding.dispatchJson(action.toJson()) }
    override fun dispatch(action: ProjectAction) = scope.launch(Dispatchers.IO) { binding.dispatchJson(action.toJson()) }
    override fun dispatch(action: CollectionAction) = scope.launch(Dispatchers.IO) { binding.dispatchJson(action.toJson()) }
    fun close() = binding.close()
}

data class CoreSnapshot(
    val home: HomeSnapshot,
    val auth: AuthSnapshot,
    val search: SearchSnapshot,
    val project: ProjectSnapshot,
    val collection: CollectionSnapshot,
)

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
private fun SearchAction.toJson(): String = when (this) {
    SearchAction.LoadHotKeys -> "{\"type\":\"search_load_hot_keys\"}"
    is SearchAction.Submit -> "{\"type\":\"search_submit\",\"keyword\":${keyword.jsonString()}}"
    is SearchAction.Author -> "{\"type\":\"search_author\",\"author\":${author.jsonString()}}"
    SearchAction.LoadNextPage -> "{\"type\":\"search_load_next_page\"}"
}
private fun ProjectAction.toJson(): String = when (this) {
    ProjectAction.LoadCategories -> "{\"type\":\"project_load_categories\"}"
    is ProjectAction.SelectCategory -> "{\"type\":\"project_select_category\",\"id\":$id}"
    ProjectAction.LoadNextPage -> "{\"type\":\"project_load_next_page\"}"
}
private fun CollectionAction.toJson(): String = when (this) {
    CollectionAction.Load -> "{\"type\":\"collection_load\"}"
    CollectionAction.Refresh -> "{\"type\":\"collection_refresh\"}"
    CollectionAction.LoadNextPage -> "{\"type\":\"collection_load_next_page\"}"
    is CollectionAction.Collect -> "{\"type\":\"collection_collect\",\"article_id\":$articleId}"
    is CollectionAction.Uncollect -> "{\"type\":\"collection_uncollect\",\"origin_id\":$originId}"
}
private fun String.jsonString(): String = buildString {
    append('"')
    for (character in this@jsonString) when (character) {
        '\\' -> append("\\\\")
        '"' -> append("\\\"")
        '\n' -> append("\\n")
        '\r' -> append("\\r")
        '\t' -> append("\\t")
        else -> append(character)
    }
    append('"')
}
