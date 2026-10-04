package com.elena.wanandroidrust.ui

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.LinearProgressIndicator
import androidx.compose.material3.pulltorefresh.PullToRefreshBox
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.elena.wanandroidrust.rust.ArticleSnapshot
import com.elena.wanandroidrust.rust.CollectionSnapshot
import com.elena.wanandroidrust.ui.components.ArticleItem
import com.elena.wanandroidrust.ui.components.InlineError
import com.elena.wanandroidrust.ui.components.ListFooter
import com.elena.wanandroidrust.ui.components.ListPadding
import com.elena.wanandroidrust.ui.components.StateMessage

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun CollectionScreen(
    state: CollectionSnapshot,
    onLoad: () -> Unit,
    onRefresh: () -> Unit,
    onMore: () -> Unit,
    onUncollect: (Long) -> Unit,
    onArticleClick: (ArticleSnapshot) -> Unit,
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
            if (state.pendingChange) {
                item(key = "pending") {
                    LinearProgressIndicator(modifier = Modifier.fillMaxWidth())
                }
            }

            val error = state.errorMessage
            if (error != null && state.articles.isEmpty()) {
                item(key = "error") {
                    StateMessage(text = error, isError = true, onRetry = onLoad)
                }
            } else if (error != null) {
                item(key = "inline-error") { InlineError(message = error) }
            } else if (state.articles.isEmpty() && !state.isLoading) {
                item(key = "empty") {
                    StateMessage(text = "收藏夹是空的，去首页点星标试试")
                }
            }

            items(state.articles, key = { it.id }) { article ->
                ArticleItem(
                    article = article.copy(collected = true),
                    onClick = { onArticleClick(article) },
                    onToggleCollect = { onUncollect(article.id) },
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
