package com.elena.wanandroidrust.ui

/**
 * Demo-only endpoint redirection.
 *
 * The API returns absolute URLs for banner images and article pages. When the shared core is
 * pointed at a different endpoint (see `BuildConfig.WANANDROID_BASE_URL`), those absolute URLs
 * still address the upstream host, which the app may not be able to reach.
 *
 * This rewrites only the scheme and authority onto the configured endpoint and keeps the path, so
 * no upstream host name is hard-coded in the Kotlin layer. When no override is configured this is
 * a no-op.
 */
object DemoEndpoint {
    @Volatile
    var baseUrl: String? = null

    fun rewrite(url: String): String {
        val base = baseUrl?.takeIf { it.isNotBlank() } ?: return url
        val schemeEnd = url.indexOf("://")
        if (schemeEnd <= 0) return url
        val authorityStart = schemeEnd + 3
        val pathStart = url.indexOf('/', authorityStart)
        val suffix = if (pathStart < 0) "" else url.substring(pathStart)
        return base.trimEnd('/') + suffix
    }
}
