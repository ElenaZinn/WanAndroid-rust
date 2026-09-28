package com.elena.wanandroidrust

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.lifecycle.lifecycleScope
import com.elena.wanandroidrust.rust.CoreJsonDecoder
import com.elena.wanandroidrust.rust.UniFfiRustCoreBinding
import com.elena.wanandroidrust.rust.NativeRustCoreGateway
import com.elena.wanandroidrust.rust.RustCoreGateway
import com.elena.wanandroidrust.ui.CollectionScreen
import com.elena.wanandroidrust.ui.HomeScreen
import com.elena.wanandroidrust.ui.LoginScreen
import com.elena.wanandroidrust.ui.MainViewModel
import com.elena.wanandroidrust.ui.ProjectScreen
import com.elena.wanandroidrust.ui.SearchScreen

class MainActivity : ComponentActivity() {
    private var nativeGateway: NativeRustCoreGateway? = null

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val binding = UniFfiRustCoreBinding()
        nativeGateway = NativeRustCoreGateway(binding, lifecycleScope, CoreJsonDecoder::decode)
        setContent { MaterialTheme { WanAndroidApp(requireNotNull(nativeGateway)) } }
    }

    override fun onDestroy() {
        nativeGateway?.close()
        super.onDestroy()
    }
}

private enum class Destination(val label: String) { HOME("Home"), SEARCH("Search"), PROJECT("Project"), COLLECTION("Saved"), ACCOUNT("Account") }

@Composable
private fun WanAndroidApp(gateway: RustCoreGateway) {
    val model = remember(gateway) { MainViewModel(gateway) }
    val home by model.homeState.collectAsState()
    val search by model.searchState.collectAsState()
    val project by model.projectState.collectAsState()
    val collection by model.collectionState.collectAsState()
    val auth by model.authState.collectAsState()
    var destination by remember { mutableStateOf(Destination.HOME) }
    var username by remember { mutableStateOf("") }
    var password by remember { mutableStateOf("") }

    Scaffold(
        bottomBar = {
            NavigationBar {
                Destination.entries.forEach { item ->
                    NavigationBarItem(
                        selected = destination == item,
                        onClick = { destination = item },
                        icon = { Text(item.label.take(1)) },
                        label = { Text(item.label) },
                    )
                }
            }
        },
    ) { padding ->
        Column(modifier = androidx.compose.ui.Modifier.padding(padding)) {
            when (destination) {
                Destination.HOME -> HomeScreen(home, model::loadHome, model::loadHomeMore, model::toggleHomeCollect)
                Destination.SEARCH -> SearchScreen(search, model::loadHotKeys, model::search, model::searchAuthor, model::loadSearchMore)
                Destination.PROJECT -> ProjectScreen(project, model::loadProjectCategories, model::selectProjectCategory, model::loadProjectMore)
                Destination.COLLECTION -> CollectionScreen(collection, model::loadCollection, model::loadCollectionMore, model::uncollect)
                Destination.ACCOUNT -> LoginScreen(username, password, auth, { username = it }, { password = it }, { model.login(username, password) }, model::logout)
            }
        }
    }
}
