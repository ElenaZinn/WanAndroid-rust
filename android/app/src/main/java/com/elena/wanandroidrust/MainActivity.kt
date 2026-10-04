package com.elena.wanandroidrust

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Home
import androidx.compose.material.icons.filled.List
import androidx.compose.material.icons.filled.Person
import androidx.compose.material.icons.filled.Search
import androidx.compose.material.icons.filled.Star
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.unit.dp
import androidx.lifecycle.lifecycleScope
import com.elena.wanandroidrust.rust.ArticleSnapshot
import com.elena.wanandroidrust.rust.CoreJsonDecoder
import com.elena.wanandroidrust.rust.NativeRustCoreGateway
import com.elena.wanandroidrust.rust.RustCoreGateway
import com.elena.wanandroidrust.rust.UniFfiRustCoreBinding
import com.elena.wanandroidrust.ui.AccountScreen
import com.elena.wanandroidrust.ui.CollectionScreen
import com.elena.wanandroidrust.ui.DemoEndpoint
import com.elena.wanandroidrust.ui.DetailScreen
import com.elena.wanandroidrust.ui.HomeScreen
import com.elena.wanandroidrust.ui.MainViewModel
import com.elena.wanandroidrust.ui.ProjectScreen
import com.elena.wanandroidrust.ui.SearchScreen
import com.elena.wanandroidrust.ui.theme.WanAndroidTheme

class MainActivity : ComponentActivity() {
    private var nativeGateway: NativeRustCoreGateway? = null

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        // Empty build config means production; a value points the shared core at a demo endpoint.
        val baseUrl = BuildConfig.WANANDROID_BASE_URL.ifBlank { null }
        // Banner images / article links arrive as absolute upstream URLs; keep them on the proxy.
        DemoEndpoint.baseUrl = baseUrl

        // Loading the Rust cdylib can fail on an unsupported ABI or a broken install. Report that
        // instead of letting the process die before the first frame.
        val startup = runCatching {
            val binding = UniFfiRustCoreBinding(baseUrl = baseUrl)
            NativeRustCoreGateway(binding, lifecycleScope, CoreJsonDecoder::decode)
        }
        nativeGateway = startup.getOrNull()

        setContent {
            WanAndroidTheme {
                val gateway = nativeGateway
                if (gateway == null) {
                    StartupFailureScreen(startup.exceptionOrNull())
                } else {
                    WanAndroidApp(gateway)
                }
            }
        }
    }

    override fun onDestroy() {
        nativeGateway?.close()
        super.onDestroy()
    }
}

@Composable
private fun StartupFailureScreen(cause: Throwable?) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(24.dp),
        verticalArrangement = Arrangement.Center,
    ) {
        Text("Rust 核心加载失败", style = MaterialTheme.typography.titleLarge)
        Spacer(Modifier.height(10.dp))
        Text(
            text = cause?.let { "${it::class.java.simpleName}: ${it.message}" }
                ?: "未知错误",
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.error,
        )
        Spacer(Modifier.height(10.dp))
        Text(
            text = "应用需要 arm64-v8a 的 libuniffi_wanandroid.so。请确认安装包完整，或重新执行 scripts/build-rust-android.sh 后重新构建。",
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
    }
}

private enum class Destination(val label: String, val icon: ImageVector) {
    HOME("首页", Icons.Filled.Home),
    SEARCH("搜索", Icons.Filled.Search),
    PROJECT("项目", Icons.Filled.List),
    COLLECTION("收藏", Icons.Filled.Star),
    ACCOUNT("我的", Icons.Filled.Person),
}

/** Article or banner currently shown in the WebView detail page. */
private data class DetailTarget(val title: String, val url: String, val articleId: Long?)

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun WanAndroidApp(gateway: RustCoreGateway) {
    val model = remember(gateway) { MainViewModel(gateway) }
    val home by model.homeState.collectAsState()
    val search by model.searchState.collectAsState()
    val project by model.projectState.collectAsState()
    val collection by model.collectionState.collectAsState()
    val auth by model.authState.collectAsState()

    var destination by remember { mutableStateOf(Destination.HOME) }
    var detail by remember { mutableStateOf<DetailTarget?>(null) }
    var username by remember { mutableStateOf("") }
    var password by remember { mutableStateOf("") }

    // Article links are absolute upstream URLs; keep them on the demo proxy when one is configured.
    val openArticle: (ArticleSnapshot) -> Unit = { article ->
        detail = DetailTarget(article.title, DemoEndpoint.rewrite(article.link), article.id)
    }

    // Restore the Rust-side session once so a saved Cookie reappears after a restart.
    LaunchedEffect(Unit) { model.restoreSession() }

    val target = detail
    if (target != null) {
        // Prefer the live snapshot so collecting from the detail page updates the star immediately.
        val live = home.articles.find { it.id == target.articleId }
            ?: collection.articles.find { it.id == target.articleId }
        DetailScreen(
            title = target.title,
            url = target.url,
            isCollected = live?.collected ?: false,
            onToggleCollect = {
                if (target.articleId != null) {
                    model.toggleHomeCollect(target.articleId)
                }
            },
            onBack = { detail = null },
        )
        return
    }

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text(destination.label) },
                colors = TopAppBarDefaults.topAppBarColors(
                    containerColor = MaterialTheme.colorScheme.surface,
                ),
            )
        },
        bottomBar = {
            NavigationBar {
                Destination.entries.forEach { item ->
                    NavigationBarItem(
                        selected = destination == item,
                        onClick = { destination = item },
                        icon = { Icon(item.icon, contentDescription = item.label) },
                        label = { Text(item.label) },
                    )
                }
            }
        },
    ) { padding ->
        Box(modifier = Modifier.padding(padding)) {
            when (destination) {
                Destination.HOME -> HomeScreen(
                    state = home,
                    onLoad = model::loadHome,
                    onRefresh = model::refreshHome,
                    onMore = model::loadHomeMore,
                    onToggleCollect = model::toggleHomeCollect,
                    onArticleClick = openArticle,
                    onBannerClick = { banner ->
                        detail = DetailTarget(banner.title, DemoEndpoint.rewrite(banner.url), null)
                    },
                )

                Destination.SEARCH -> SearchScreen(
                    state = search,
                    onLoadHotKeys = model::loadHotKeys,
                    onSearch = model::search,
                    onSearchAuthor = model::searchAuthor,
                    onMore = model::loadSearchMore,
                    onArticleClick = openArticle,
                    onToggleCollect = model::toggleHomeCollect,
                )

                Destination.PROJECT -> ProjectScreen(
                    state = project,
                    onLoadCategories = model::loadProjectCategories,
                    onSelectCategory = model::selectProjectCategory,
                    onMore = model::loadProjectMore,
                    onArticleClick = openArticle,
                    onToggleCollect = model::toggleHomeCollect,
                )

                Destination.COLLECTION -> CollectionScreen(
                    state = collection,
                    onLoad = model::loadCollection,
                    onRefresh = model::refreshCollection,
                    onMore = model::loadCollectionMore,
                    onUncollect = model::uncollect,
                    onArticleClick = openArticle,
                )

                Destination.ACCOUNT -> AccountScreen(
                    username = username,
                    password = password,
                    state = auth,
                    onUsernameChange = { username = it },
                    onPasswordChange = { password = it },
                    onLogin = { model.login(username, password) },
                    onLogout = model::logout,
                )
            }
        }
    }
}
