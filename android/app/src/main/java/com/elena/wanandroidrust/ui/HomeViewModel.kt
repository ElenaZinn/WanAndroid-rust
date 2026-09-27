package com.elena.wanandroidrust.ui

import androidx.lifecycle.ViewModel
import com.elena.wanandroidrust.rust.HomeAction
import com.elena.wanandroidrust.rust.RustCoreGateway

class HomeViewModel(private val gateway: RustCoreGateway) : ViewModel() {
    val state = gateway.homeState

    fun onAppear() = gateway.dispatch(HomeAction.Load)
    fun onRefresh() = gateway.dispatch(HomeAction.Refresh)
    fun onLoadMore() = gateway.dispatch(HomeAction.LoadNextPage)
    fun onToggleCollect(articleId: Long) = gateway.dispatch(HomeAction.ToggleCollect(articleId))
}
