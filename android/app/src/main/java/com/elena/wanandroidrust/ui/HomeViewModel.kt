package com.elena.wanandroidrust.ui

import androidx.lifecycle.ViewModel
import com.elena.wanandroidrust.rust.AuthAction
import com.elena.wanandroidrust.rust.HomeAction
import com.elena.wanandroidrust.rust.RustCoreGateway

class HomeViewModel(private val gateway: RustCoreGateway) : ViewModel() {
    val state = gateway.homeState
    val authState = gateway.authState

    fun onAppear() = gateway.dispatch(HomeAction.Load)
    fun onRefresh() = gateway.dispatch(HomeAction.Refresh)
    fun onLoadMore() = gateway.dispatch(HomeAction.LoadNextPage)
    fun onToggleCollect(articleId: Long) = gateway.dispatch(HomeAction.ToggleCollect(articleId))
    fun onRestoreSession() = gateway.dispatch(AuthAction.Restore)
    fun onLogin(username: String, password: String) = gateway.dispatch(AuthAction.Login(username, password))
    fun onLogout() = gateway.dispatch(AuthAction.Logout)
}
