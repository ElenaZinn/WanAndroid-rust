package com.elena.wanandroidrust.ui

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.pulltorefresh.PullToRefreshBox
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.elena.wanandroidrust.rust.ArticleSnapshot
import com.elena.wanandroidrust.rust.BannerSnapshot
import com.elena.wanandroidrust.rust.HomeSnapshot
import com.elena.wanandroidrust.ui.components.ArticleItem
import com.elena.wanandroidrust.ui.components.BannerCarousel
import com.elena.wanandroidrust.ui.components.InlineError
import com.elena.wanandroidrust.ui.components.ListFooter
import com.elena.wanandroidrust.ui.components.ListPadding
import com.elena.wanandroidrust.ui.components.StateMessage

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun HomeScreen(
    state: HomeSnapshot,
    onLoad: () -> Unit,
    onRefresh: () -> Unit,
    onMore: () -> Unit,
    onToggleCollect: (Long) -> Unit,
    onArticleClick: (ArticleSnapshot) -> Unit,
    onBannerClick: (BannerSnapshot) -> Unit,
) {
    LaunchedEffect(Unit) { onLoad() }

    PullToRefreshBox(
        isRefreshing = state.isLoading && state.articles.isNotEmpty(),
        onRefresh = onRefresh,
        modifier = Modifier.fillMaxSize(),
    ) {
        LazyColumn(
            modifier = Modifier.fillMaxSize(),
            contentPadding = ListPadding,
            verticalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            if (state.banners.isNotEmpty()) {
                item(key = "banners") {
                    BannerCarousel(banners = state.banners, onBannerClick = onBannerClick)
                }
            }

            val error = state.errorMessage
            if (error != null && state.articles.isEmpty()) {
                item(key = "error") {
                    StateMessage(text = error, isError = true, onRetry = onLoad)
                }
            } else if (error != null) {
                // Keep the list visible; report the failure (for example a failed collect) inline.
                item(key = "inline-error") { InlineError(message = error) }
            } else if (state.articles.isEmpty() && !state.isLoading) {
                item(key = "empty") { StateMessage(text = "暂无内容，下拉刷新试试") }
            }

            items(state.articles, key = { it.id }) { article ->
                ArticleItem(
                    article = article,
                    onClick = { onArticleClick(article) },
                    onToggleCollect = { onToggleCollect(article.id) },
                )
            }

            item(key = "footer") {
                ListFooter(
                    isLoading = state.isLoading,
                    canLoadMore = state.canLoadMore,
                    itemCount = state.articles.size,
                    onLoadMore = onMore,
                    isError = state.errorMessage != null,
                )
            }
        }
    }
}
