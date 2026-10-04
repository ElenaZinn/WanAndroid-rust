package com.elena.wanandroidrust.ui

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.Button
import androidx.compose.material3.FilterChip
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.unit.dp
import com.elena.wanandroidrust.rust.ArticleSnapshot
import com.elena.wanandroidrust.rust.SearchSnapshot
import com.elena.wanandroidrust.ui.components.ArticleItem
import com.elena.wanandroidrust.ui.components.InlineError
import com.elena.wanandroidrust.ui.components.ListFooter
import com.elena.wanandroidrust.ui.components.ListPadding
import com.elena.wanandroidrust.ui.components.StateMessage

private enum class SearchMode(val label: String) {
    KEYWORD("关键词"),
    AUTHOR("作者"),
}

@OptIn(ExperimentalLayoutApi::class)
@Composable
fun SearchScreen(
    state: SearchSnapshot,
    onLoadHotKeys: () -> Unit,
    onSearch: (String) -> Unit,
    onSearchAuthor: (String) -> Unit,
    onMore: () -> Unit,
    onArticleClick: (ArticleSnapshot) -> Unit,
    onToggleCollect: (Long) -> Unit,
) {
    var query by remember { mutableStateOf("") }
    var mode by remember { mutableStateOf(SearchMode.KEYWORD) }

    LaunchedEffect(Unit) { onLoadHotKeys() }

    val submit = {
        when (mode) {
            SearchMode.KEYWORD -> onSearch(query)
            SearchMode.AUTHOR -> onSearchAuthor(query)
        }
        Unit
    }

    Column(modifier = Modifier.fillMaxSize()) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 12.dp, vertical = 10.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                SearchMode.entries.forEach { entry ->
                    FilterChip(
                        selected = mode == entry,
                        onClick = { mode = entry },
                        label = { Text(entry.label) },
                    )
                }
            }
            Row(
                horizontalArrangement = Arrangement.spacedBy(8.dp),
                verticalAlignment = androidx.compose.ui.Alignment.CenterVertically,
            ) {
                OutlinedTextField(
                    value = query,
                    onValueChange = { query = it },
                    singleLine = true,
                    label = { Text(if (mode == SearchMode.KEYWORD) "搜索关键词" else "搜索作者") },
                    keyboardOptions = KeyboardOptions(imeAction = ImeAction.Search),
                    keyboardActions = KeyboardActions(onSearch = { submit() }),
                    modifier = Modifier.weight(1f),
                )
                Button(onClick = submit) { Text("搜索") }
            }
        }

        if (state.hotKeys.isNotEmpty() && state.articles.isEmpty() && !state.isLoading) {
            Column(modifier = Modifier.padding(horizontal = 12.dp)) {
                Text(
                    text = "热门搜索",
                    style = MaterialTheme.typography.titleSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
                Spacer(Modifier.height(6.dp))
                FlowRow(
                    horizontalArrangement = Arrangement.spacedBy(8.dp),
                    verticalArrangement = Arrangement.spacedBy(4.dp),
                ) {
                    state.hotKeys.forEach { hotKey ->
                        FilterChip(
                            selected = false,
                            onClick = {
                                query = hotKey.name
                                onSearch(hotKey.name)
                            },
                            label = { Text(hotKey.name) },
                        )
                    }
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
                item(key = "error") { StateMessage(text = error, isError = true) }
            } else if (error != null) {
                item(key = "inline-error") { InlineError(message = error) }
            } else if (state.articles.isEmpty() && !state.isLoading && state.hotKeys.isEmpty()) {
                item(key = "empty") { StateMessage(text = "输入关键词或作者开始搜索") }
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
