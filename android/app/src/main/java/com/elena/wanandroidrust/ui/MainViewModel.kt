package com.elena.wanandroidrust.ui

import androidx.lifecycle.ViewModel
import com.elena.wanandroidrust.rust.AuthAction
import com.elena.wanandroidrust.rust.CollectionAction
import com.elena.wanandroidrust.rust.HomeAction
import com.elena.wanandroidrust.rust.ProjectAction
import com.elena.wanandroidrust.rust.RustCoreGateway
import com.elena.wanandroidrust.rust.SearchAction

class MainViewModel(private val gateway: RustCoreGateway) : ViewModel() {
    val homeState = gateway.homeState
    val authState = gateway.authState
    val searchState = gateway.searchState
    val projectState = gateway.projectState
    val collectionState = gateway.collectionState

    fun loadHome() = gateway.dispatch(HomeAction.Load)
    fun refreshHome() = gateway.dispatch(HomeAction.Refresh)
    fun loadHomeMore() = gateway.dispatch(HomeAction.LoadNextPage)
    fun toggleHomeCollect(id: Long) = gateway.dispatch(HomeAction.ToggleCollect(id))

    fun restoreSession() = gateway.dispatch(AuthAction.Restore)
    fun login(username: String, password: String) = gateway.dispatch(AuthAction.Login(username, password))
    fun logout() = gateway.dispatch(AuthAction.Logout)

    fun loadHotKeys() = gateway.dispatch(SearchAction.LoadHotKeys)
    fun search(keyword: String) = gateway.dispatch(SearchAction.Submit(keyword))
    fun searchAuthor(author: String) = gateway.dispatch(SearchAction.Author(author))
    fun loadSearchMore() = gateway.dispatch(SearchAction.LoadNextPage)

    fun loadProjectCategories() = gateway.dispatch(ProjectAction.LoadCategories)
    fun selectProjectCategory(id: Long) = gateway.dispatch(ProjectAction.SelectCategory(id))
    fun loadProjectMore() = gateway.dispatch(ProjectAction.LoadNextPage)

    fun loadCollection() = gateway.dispatch(CollectionAction.Load)
    fun refreshCollection() = gateway.dispatch(CollectionAction.Refresh)
    fun loadCollectionMore() = gateway.dispatch(CollectionAction.LoadNextPage)
    fun collect(id: Long) = gateway.dispatch(CollectionAction.Collect(id))
    fun uncollect(id: Long) = gateway.dispatch(CollectionAction.Uncollect(id))
}
