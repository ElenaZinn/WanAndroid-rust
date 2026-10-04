package com.elena.wanandroidrust.ui.components

import android.text.Html
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.pager.HorizontalPager
import androidx.compose.foundation.pager.rememberPagerState
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Refresh
import androidx.compose.material.icons.filled.Star
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import coil.compose.AsyncImage
import com.elena.wanandroidrust.rust.ArticleSnapshot
import com.elena.wanandroidrust.rust.BannerSnapshot
import com.elena.wanandroidrust.ui.DemoEndpoint
import com.elena.wanandroidrust.ui.theme.TagContainer
import com.elena.wanandroidrust.ui.theme.TagText
import kotlinx.coroutines.delay

/** Article date values arrive as `yyyy-MM-dd HH:mm:ss`; the list only needs the date part. */
fun String.datePart(): String = take(10)

fun String.orAnonymous(): String = ifBlank { "匿名" }

/**
 * Search results wrap the matched keyword in `<em>` tags and escape punctuation as entities such
 * as `&mdash;`. The UI shows plain text, so decode both through the platform HTML helper.
 */
fun String.stripHtml(): String =
    if ('<' in this || '&' in this) {
        Html.fromHtml(this, Html.FROM_HTML_MODE_LEGACY).toString()
    } else {
        this
    }

@Composable
fun CategoryTag(text: String, modifier: Modifier = Modifier) {
    if (text.isBlank()) return
    Text(
        text = text,
        style = MaterialTheme.typography.labelSmall,
        color = TagText,
        maxLines = 1,
        overflow = TextOverflow.Ellipsis,
        modifier = modifier
            .clip(RoundedCornerShape(4.dp))
            .background(TagContainer)
            .padding(horizontal = 6.dp, vertical = 2.dp),
    )
}

@Composable
fun ArticleItem(
    article: ArticleSnapshot,
    onClick: () -> Unit,
    onToggleCollect: () -> Unit,
    modifier: Modifier = Modifier,
) {
    Card(
        modifier = modifier
            .fillMaxWidth()
            .clickable(onClick = onClick),
        shape = RoundedCornerShape(8.dp),
        colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.surface),
        elevation = CardDefaults.cardElevation(defaultElevation = 0.dp),
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(12.dp),
            verticalAlignment = Alignment.Top,
        ) {
            Column(modifier = Modifier.weight(1f)) {
                Row(verticalAlignment = Alignment.CenterVertically) {
                    CategoryTag(article.chapterName)
                    Spacer(Modifier.size(6.dp))
                    Text(
                        text = article.title.stripHtml(),
                        style = MaterialTheme.typography.bodyLarge,
                        fontWeight = FontWeight.Medium,
                        maxLines = 2,
                        overflow = TextOverflow.Ellipsis,
                        modifier = Modifier.weight(1f, fill = false),
                    )
                }
                Spacer(Modifier.height(6.dp))
                Text(
                    text = "${article.author.orAnonymous()} · ${article.date.datePart()}",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                    maxLines = 1,
                    overflow = TextOverflow.Ellipsis,
                )
            }
            IconButton(onClick = onToggleCollect) {
                Icon(
                    imageVector = Icons.Filled.Star,
                    contentDescription = if (article.collected) "取消收藏" else "收藏",
                    tint = if (article.collected) {
                        MaterialTheme.colorScheme.primary
                    } else {
                        MaterialTheme.colorScheme.outlineVariant
                    },
                )
            }
        }
    }
}

@Composable
fun BannerCarousel(
    banners: List<BannerSnapshot>,
    onBannerClick: (BannerSnapshot) -> Unit,
    modifier: Modifier = Modifier,
) {
    if (banners.isEmpty()) return

    val pagerState = rememberPagerState(pageCount = { banners.size })

    // Auto-advance like the original home banner. Restarts whenever the banner list changes.
    LaunchedEffect(pagerState, banners.size) {
        if (banners.size < 2) return@LaunchedEffect
        while (true) {
            delay(4_000)
            val next = (pagerState.currentPage + 1) % banners.size
            pagerState.animateScrollToPage(next)
        }
    }

    Box(modifier = modifier.fillMaxWidth()) {
        HorizontalPager(
            state = pagerState,
            modifier = Modifier
                .fillMaxWidth()
                .aspectRatio(16f / 7f),
            pageSpacing = 8.dp,
        ) { page ->
            val banner = banners[page]
            Card(
                shape = RoundedCornerShape(8.dp),
                colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.surfaceVariant),
                modifier = Modifier
                    .fillMaxSize()
                    .clickable { onBannerClick(banner) },
            ) {
                Box(modifier = Modifier.fillMaxSize()) {
                    AsyncImage(
                        model = DemoEndpoint.rewrite(banner.imagePath),
                        contentDescription = banner.title,
                        contentScale = ContentScale.Crop,
                        modifier = Modifier.fillMaxSize(),
                    )
                    if (banner.title.isNotBlank()) {
                        Text(
                            text = banner.title,
                            style = MaterialTheme.typography.labelLarge,
                            color = Color.White,
                            maxLines = 1,
                            overflow = TextOverflow.Ellipsis,
                            modifier = Modifier
                                .align(Alignment.BottomStart)
                                .fillMaxWidth()
                                .background(Color(0x99000000))
                                .padding(horizontal = 10.dp, vertical = 6.dp),
                        )
                    }
                }
            }
        }

        if (banners.size > 1) {
            Row(
                modifier = Modifier
                    .align(Alignment.BottomEnd)
                    .padding(end = 12.dp, bottom = 34.dp),
                horizontalArrangement = Arrangement.spacedBy(4.dp),
            ) {
                repeat(banners.size) { index ->
                    Box(
                        modifier = Modifier
                            .size(if (index == pagerState.currentPage) 7.dp else 5.dp)
                            .clip(CircleShape)
                            .background(
                                if (index == pagerState.currentPage) {
                                    Color.White
                                } else {
                                    Color(0x80FFFFFF)
                                },
                            ),
                    )
                }
            }
        }
    }
}

/**
 * Footer for paginated lists. Loads the next page when it becomes visible, and keeps an explicit
 * retry affordance so a failed page does not leave the list stuck.
 */
@Composable
fun ListFooter(
    isLoading: Boolean,
    canLoadMore: Boolean,
    itemCount: Int,
    onLoadMore: () -> Unit,
    isError: Boolean = false,
) {
    if (itemCount == 0) return

    LaunchedEffect(canLoadMore, isLoading, isError, itemCount) {
        if (canLoadMore && !isLoading && !isError) onLoadMore()
    }

    Box(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 14.dp),
        contentAlignment = Alignment.Center,
    ) {
        when {
            isLoading -> CircularProgressIndicator(modifier = Modifier.size(22.dp), strokeWidth = 2.dp)
            isError -> TextButton(onClick = onLoadMore) {
                Icon(Icons.Filled.Refresh, contentDescription = null, modifier = Modifier.size(16.dp))
                Spacer(Modifier.size(4.dp))
                Text("重新加载")
            }
            !canLoadMore -> Text(
                text = "没有更多了",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
    }
}

/** Inline failure banner for a list that already has content, so the list is not replaced. */
@Composable
fun InlineError(
    message: String,
    modifier: Modifier = Modifier,
    onRetry: (() -> Unit)? = null,
) {
    Row(
        modifier = modifier
            .fillMaxWidth()
            .clip(RoundedCornerShape(8.dp))
            .background(MaterialTheme.colorScheme.errorContainer)
            .padding(horizontal = 12.dp, vertical = 10.dp),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Text(
            text = message,
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onErrorContainer,
            modifier = Modifier.weight(1f),
        )
        if (onRetry != null) {
            TextButton(onClick = onRetry) { Text("重试") }
        }
    }
}

@Composable
fun StateMessage(
    text: String,
    modifier: Modifier = Modifier,
    isError: Boolean = false,
    onRetry: (() -> Unit)? = null,
) {
    Column(
        modifier = modifier
            .fillMaxWidth()
            .padding(horizontal = 24.dp, vertical = 40.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
    ) {
        Text(
            text = text,
            style = MaterialTheme.typography.bodyMedium,
            color = if (isError) MaterialTheme.colorScheme.error else MaterialTheme.colorScheme.onSurfaceVariant,
        )
        if (onRetry != null) {
            Spacer(Modifier.height(10.dp))
            TextButton(onClick = onRetry) {
                Icon(Icons.Filled.Refresh, contentDescription = null, modifier = Modifier.size(16.dp))
                Spacer(Modifier.size(4.dp))
                Text("重试")
            }
        }
    }
}

/** Shared list padding so every tab lines up the same way. */
val ListPadding = PaddingValues(horizontal = 12.dp, vertical = 8.dp)
