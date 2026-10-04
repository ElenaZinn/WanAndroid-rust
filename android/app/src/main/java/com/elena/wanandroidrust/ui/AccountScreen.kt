package com.elena.wanandroidrust.ui

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.unit.dp
import com.elena.wanandroidrust.rust.AuthSnapshot

@Composable
fun AccountScreen(
    username: String,
    password: String,
    state: AuthSnapshot,
    onUsernameChange: (String) -> Unit,
    onPasswordChange: (String) -> Unit,
    onLogin: () -> Unit,
    onLogout: () -> Unit,
) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        Card(
            modifier = Modifier.fillMaxWidth(),
            colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.surface),
        ) {
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(16.dp),
                verticalArrangement = Arrangement.spacedBy(12.dp),
            ) {
                val signedIn = state.username
                if (signedIn != null) {
                    Text(
                        text = "已登录：$signedIn",
                        style = MaterialTheme.typography.titleMedium,
                    )
                    Text(
                        text = "登录态保存在 Rust 侧的内存会话中，Cookie 不经过 Kotlin。",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                    Button(onClick = onLogout) { Text("退出登录") }
                } else {
                    Text(text = "登录 WanAndroid", style = MaterialTheme.typography.titleMedium)
                    OutlinedTextField(
                        value = username,
                        onValueChange = onUsernameChange,
                        singleLine = true,
                        label = { Text("用户名") },
                        keyboardOptions = KeyboardOptions(imeAction = ImeAction.Next),
                        modifier = Modifier.fillMaxWidth(),
                    )
                    OutlinedTextField(
                        value = password,
                        onValueChange = onPasswordChange,
                        singleLine = true,
                        label = { Text("密码") },
                        visualTransformation = PasswordVisualTransformation(),
                        keyboardOptions = KeyboardOptions(imeAction = ImeAction.Done),
                        keyboardActions = KeyboardActions(onDone = { onLogin() }),
                        modifier = Modifier.fillMaxWidth(),
                    )
                    Button(
                        onClick = onLogin,
                        enabled = username.isNotBlank() && password.isNotBlank() && !state.isLoading,
                        modifier = Modifier.fillMaxWidth(),
                    ) {
                        Text("登录")
                    }
                }

                if (state.isLoading) {
                    CircularProgressIndicator(modifier = Modifier.padding(top = 4.dp))
                }

                state.errorMessage?.let { message ->
                    Text(
                        text = message,
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.error,
                    )
                    TextButton(onClick = onLogin) { Text("重试") }
                }
            }
        }
    }
}
