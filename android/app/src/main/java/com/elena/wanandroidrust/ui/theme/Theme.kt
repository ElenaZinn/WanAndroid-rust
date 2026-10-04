package com.elena.wanandroidrust.ui.theme

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.Color

// Palette kept close to the original WanAndroid client: blue accent, light grey page background.
private val BrandBlue = Color(0xFF1F6FEB)
private val BrandBlueDark = Color(0xFF0B4CB0)
private val PageBackground = Color(0xFFF5F6F8)
private val CardSurface = Color(0xFFFFFFFF)
private val ChipBlue = Color(0xFFE8F0FE)
private val ChipBlueText = Color(0xFF1F6FEB)

private val LightColors = lightColorScheme(
    primary = BrandBlue,
    onPrimary = Color.White,
    primaryContainer = ChipBlue,
    onPrimaryContainer = BrandBlueDark,
    secondary = Color(0xFF4A6588),
    onSecondary = Color.White,
    background = PageBackground,
    onBackground = Color(0xFF1B1D21),
    surface = CardSurface,
    onSurface = Color(0xFF1B1D21),
    surfaceVariant = Color(0xFFEDEFF3),
    onSurfaceVariant = Color(0xFF5A6070),
    outlineVariant = Color(0xFFE2E5EA),
    error = Color(0xFFD93025),
)

private val DarkColors = darkColorScheme(
    primary = Color(0xFF7FB0FF),
    onPrimary = Color(0xFF06213F),
    primaryContainer = Color(0xFF143A66),
    onPrimaryContainer = Color(0xFFD7E6FF),
    secondary = Color(0xFFB3C6DE),
    background = Color(0xFF121417),
    surface = Color(0xFF1B1E23),
    surfaceVariant = Color(0xFF272B31),
    error = Color(0xFFFF8A80),
)

/** Tag colour used by article category chips; matches the blue tag look of the original app. */
val TagContainer: Color
    @Composable get() = if (isSystemInDarkTheme()) Color(0xFF1E3A5F) else ChipBlue

val TagText: Color
    @Composable get() = if (isSystemInDarkTheme()) Color(0xFFB7D3FF) else ChipBlueText

@Composable
fun WanAndroidTheme(content: @Composable () -> Unit) {
    MaterialTheme(
        colorScheme = if (isSystemInDarkTheme()) DarkColors else LightColors,
        content = content,
    )
}
