package com.elena.wanandroidrust.ui

/**
 * Demo-only endpoint redirection.
 *
 * The API returns absolute URLs for banner images and article links. When the shared core is
 * pointed at a local demo proxy (see `BuildConfig.WANANDROID_BASE_URL`), those absolute URLs still
 * address the upstream host, which the app cannot reach. This rewrites them onto the same base URL
 * the core is using so images and article pages load through the proxy too.
 *
 * When no override is configured this is a no-op and the app behaves normally.
 */
object DemoEndpoint {
    private val upstreamHosts = listOf(
        "https://www.wanandroid.com",
        "https://wanandroid.com",
        "http://www.wanandroid.com",
        "http://wanandroid.com",
    )

    @Volatile
    var baseUrl: String? = null

    fun rewrite(url: String): String {
        val base = baseUrl?.takeIf { it.isNotBlank() } ?: return url
        for (host in upstreamHosts) {
            if (url.startsWith(host)) {
                return base.trimEnd('/') + url.removePrefix(host)
            }
        }
        return url
    }
}
