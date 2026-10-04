package com.elena.wanandroidrust.ui

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.ScrollableTabRow
import androidx.compose.material3.Tab
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.elena.wanandroidrust.rust.ArticleSnapshot
import com.elena.wanandroidrust.rust.ProjectArticleSnapshot
import com.elena.wanandroidrust.rust.ProjectSnapshot
import com.elena.wanandroidrust.ui.components.ArticleItem
import com.elena.wanandroidrust.ui.components.InlineError
import com.elena.wanandroidrust.ui.components.ListFooter
import com.elena.wanandroidrust.ui.components.ListPadding
import com.elena.wanandroidrust.ui.components.StateMessage

/** Project articles carry two extra image fields the shared list item does not need. */
private fun ProjectArticleSnapshot.asArticle(): ArticleSnapshot = ArticleSnapshot(
    id = id,
    title = title,
    link = link,
    author = author,
    date = date,
    collected = collected,
    chapterName = chapterName,
)

@Composable
fun ProjectScreen(
    state: ProjectSnapshot,
    onLoadCategories: () -> Unit,
    onSelectCategory: (Long) -> Unit,
    onMore: () -> Unit,
    onArticleClick: (ArticleSnapshot) -> Unit,
    onToggleCollect: (Long) -> Unit,
) {
    LaunchedEffect(Unit) { onLoadCategories() }

    Column(modifier = Modifier.fillMaxSize()) {
        if (state.categories.isNotEmpty()) {
            val selectedIndex = state.categories
                .indexOfFirst { it.id == state.selectedCategory }
                .coerceAtLeast(0)
            ScrollableTabRow(selectedTabIndex = selectedIndex, edgePadding = 12.dp) {
                state.categories.forEachIndexed { index, category ->
                    Tab(
                        selected = index == selectedIndex,
                        onClick = { onSelectCategory(category.id) },
                        text = { Text(category.name) },
                    )
                }
            }
        }

        LazyColumn(
            modifier = Modifier.fillMaxSize(),
            contentPadding = ListPadding,
            verticalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            val error = state.errorMessage
            if (error != null && state.articles.isEmpty()) {
                item(key = "error") {
                    StateMessage(text = error, isError = true, onRetry = onLoadCategories)
                }
            } else if (error != null) {
                item(key = "inline-error") { InlineError(message = error) }
            } else if (state.articles.isEmpty() && !state.isLoading) {
                item(key = "empty") { StateMessage(text = "该分类下暂无项目") }
            }

            items(state.articles, key = { it.id }) { projectArticle ->
                val article = projectArticle.asArticle()
                ArticleItem(
                    article = article,
                    onClick = { onArticleClick(article) },
                    onToggleCollect = { onToggleCollect(projectArticle.id) },
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
