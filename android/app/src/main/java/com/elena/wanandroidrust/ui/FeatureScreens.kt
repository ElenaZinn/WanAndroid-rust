package com.elena.wanandroidrust.ui

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.CircularProgressIndicator
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
import androidx.compose.ui.unit.dp
import com.elena.wanandroidrust.rust.ArticleSnapshot
import com.elena.wanandroidrust.rust.CollectionSnapshot
import com.elena.wanandroidrust.rust.HomeSnapshot
import com.elena.wanandroidrust.rust.ProjectSnapshot
import com.elena.wanandroidrust.rust.SearchSnapshot

@Composable
fun HomeScreen(state: HomeSnapshot, onLoad: () -> Unit, onMore: () -> Unit, onCollect: (Long) -> Unit) {
    LaunchedEffect(Unit) { onLoad() }
    FeatureColumn(title = "Home", loading = state.isLoading, error = state.errorMessage) {
        state.banners.forEach { Text(it.title, style = MaterialTheme.typography.titleMedium) }
        ArticleList(state.articles, state.canLoadMore, onMore, onCollect)
    }
}

@Composable
fun SearchScreen(state: SearchSnapshot, onHotKeys: () -> Unit, onSearch: (String) -> Unit, onAuthor: (String) -> Unit, onMore: () -> Unit) {
    var keyword by remember { mutableStateOf(state.keyword) }
    var author by remember { mutableStateOf("") }
    LaunchedEffect(Unit) { onHotKeys() }
    FeatureColumn(title = "Search", loading = state.isLoading, error = state.errorMessage) {
        OutlinedTextField(value = keyword, onValueChange = { keyword = it }, label = { Text("Keyword") }, modifier = Modifier.fillMaxWidth())
        Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            Button(onClick = { onSearch(keyword) }) { Text("Search") }
            Button(onClick = { onAuthor(author) }) { Text("By author") }
        }
        OutlinedTextField(value = author, onValueChange = { author = it }, label = { Text("Author") }, modifier = Modifier.fillMaxWidth())
        state.hotKeys.take(8).forEach { FilterChip(selected = keyword == it.name, onClick = { keyword = it.name }, label = { Text(it.name) }) }
        ArticleList(state.articles, state.canLoadMore, onMore, {})
    }
}

@Composable
fun ProjectScreen(state: ProjectSnapshot, onLoad: () -> Unit, onSelect: (Long) -> Unit, onMore: () -> Unit) {
    LaunchedEffect(Unit) { onLoad() }
    FeatureColumn(title = "Project", loading = state.isLoading, error = state.errorMessage) {
        Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
            state.categories.forEach { category ->
                FilterChip(selected = state.selectedCategory == category.id, onClick = { onSelect(category.id) }, label = { Text(category.name) })
            }
        }
        state.articles.forEach { article -> Card(Modifier.fillMaxWidth()) { Text(article.title, Modifier.padding(12.dp)) } }
        if (state.canLoadMore) Button(onClick = onMore) { Text("Load more") }
    }
}

@Composable
fun LoginScreen(username: String, password: String, state: com.elena.wanandroidrust.rust.AuthSnapshot, onUsername: (String) -> Unit, onPassword: (String) -> Unit, onLogin: () -> Unit, onLogout: () -> Unit) {
    FeatureColumn(title = "Account", loading = state.isLoading, error = state.errorMessage) {
        if (state.username != null) {
            Text("Signed in as ${state.username}")
            Button(onClick = onLogout) { Text("Log out") }
        } else {
            OutlinedTextField(value = username, onValueChange = onUsername, label = { Text("Username") }, modifier = Modifier.fillMaxWidth())
            OutlinedTextField(value = password, onValueChange = onPassword, label = { Text("Password") }, modifier = Modifier.fillMaxWidth())
            Button(onClick = onLogin) { Text("Log in") }
        }
    }
}

@Composable
fun CollectionScreen(state: CollectionSnapshot, onLoad: () -> Unit, onMore: () -> Unit, onUncollect: (Long) -> Unit) {
    LaunchedEffect(Unit) { onLoad() }
    FeatureColumn(title = "Collection", loading = state.isLoading, error = state.errorMessage) {
        state.articles.forEach { article ->
            Card(Modifier.fillMaxWidth()) {
                Row(Modifier.fillMaxWidth().padding(12.dp), horizontalArrangement = Arrangement.SpaceBetween) {
                    Text(article.title, Modifier.weight(1f))
                    Button(onClick = { onUncollect(article.id) }) { Text("Remove") }
                }
            }
        }
        if (state.canLoadMore) Button(onClick = onMore) { Text("Load more") }
    }
}

@Composable
private fun FeatureColumn(title: String, loading: Boolean, error: String?, content: @Composable () -> Unit) {
    Column(Modifier.fillMaxWidth().padding(horizontal = 16.dp), verticalArrangement = Arrangement.spacedBy(10.dp)) {
        Text(title, style = MaterialTheme.typography.headlineSmall)
        if (loading) CircularProgressIndicator()
        error?.let { Text("Error: $it", color = MaterialTheme.colorScheme.error) }
        content()
    }
}

@Composable
private fun ArticleList(articles: List<ArticleSnapshot>, canLoadMore: Boolean, onMore: () -> Unit, onCollect: (Long) -> Unit) {
    LazyColumn(contentPadding = PaddingValues(vertical = 8.dp), verticalArrangement = Arrangement.spacedBy(8.dp)) {
        items(articles, key = { it.id }) { article ->
            Card(Modifier.fillMaxWidth()) {
                Row(Modifier.fillMaxWidth().padding(12.dp), horizontalArrangement = Arrangement.SpaceBetween) {
                    Column(Modifier.weight(1f)) {
                        Text(article.title, style = MaterialTheme.typography.titleMedium)
                        Text("${article.author} · ${article.date}", style = MaterialTheme.typography.bodySmall)
                    }
                    Button(onClick = { onCollect(article.id) }) { Text(if (article.collected) "Saved" else "Save") }
                }
            }
        }
        if (canLoadMore) item { Button(onClick = onMore) { Text("Load more") } }
    }
}
