package com.elena.wanandroidrust

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.elena.wanandroidrust.rust.AuthAction
import com.elena.wanandroidrust.rust.AuthSnapshot
import com.elena.wanandroidrust.rust.HomeAction
import com.elena.wanandroidrust.rust.HomeSnapshot
import com.elena.wanandroidrust.rust.RustCoreGateway

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            MaterialTheme {
                HomeScreen(PreviewRustCoreGateway())
            }
        }
    }
}

@Composable
private fun HomeScreen(gateway: RustCoreGateway) {
    val state by gateway.homeState.collectAsState()
    Column(
        modifier = Modifier.fillMaxSize().padding(24.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp),
    ) {
        Text("WanAndroid Rust", style = MaterialTheme.typography.headlineMedium)
        when {
            state.isLoading -> CircularProgressIndicator()
            state.errorMessage != null -> Text("Error: ${state.errorMessage}")
            state.articles.isEmpty() -> Text("No articles yet")
            else -> state.articles.forEach { Text(it.title) }
        }
        Button(onClick = { gateway.dispatch(com.elena.wanandroidrust.rust.HomeAction.Load) }) {
            Text("Load from Rust core")
        }
    }
}

private class PreviewRustCoreGateway : RustCoreGateway {
    private val _state = kotlinx.coroutines.flow.MutableStateFlow(HomeSnapshot())
    private val _authState = kotlinx.coroutines.flow.MutableStateFlow(AuthSnapshot())
    override val homeState = _state
    override val authState = _authState

    override fun dispatch(action: HomeAction) {
        if (action is HomeAction.Load) {
            _state.value = HomeSnapshot(isLoading = true)
        }
    }

    override fun dispatch(action: AuthAction) {
        if (action is AuthAction.Restore) {
            _authState.value = AuthSnapshot()
        }
    }
}
