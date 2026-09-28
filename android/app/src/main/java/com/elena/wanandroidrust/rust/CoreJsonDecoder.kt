package com.elena.wanandroidrust.rust

import org.json.JSONArray
import org.json.JSONObject

object CoreJsonDecoder {
    fun decode(raw: String): CoreSnapshot {
        val root = JSONObject(raw)
        return CoreSnapshot(
            home = home(root.optJSONObject("home")),
            auth = auth(root.optJSONObject("auth")),
            search = search(root.optJSONObject("search")),
            project = project(root.optJSONObject("project")),
            collection = collection(root.optJSONObject("collection")),
        )
    }

    private fun home(json: JSONObject?): HomeSnapshot = HomeSnapshot(
        banners = json?.array("banners") { banner(it) } ?: emptyList(),
        articles = json?.array("articles") { article(it) } ?: emptyList(),
        isLoading = json?.optBoolean("is_loading") ?: false,
        errorMessage = json?.nullableString("error_message"),
        canLoadMore = json?.optBoolean("can_load_more", true) ?: true,
    )

    private fun auth(json: JSONObject?): AuthSnapshot = AuthSnapshot(
        username = json?.nullableString("username"),
        isLoading = json?.optBoolean("is_loading") ?: false,
        errorMessage = json?.nullableString("error_message"),
    )

    private fun search(json: JSONObject?): SearchSnapshot = SearchSnapshot(
        hotKeys = json?.array("hot_keys") { HotKeySnapshot(it.optLong("id"), it.optString("name"), it.optString("link")) } ?: emptyList(),
        articles = json?.array("articles") { article(it) } ?: emptyList(),
        keyword = json?.optString("keyword").orEmpty(),
        author = json?.nullableString("author"),
        isLoading = json?.optBoolean("is_loading") ?: false,
        errorMessage = json?.nullableString("error_message"),
        canLoadMore = json?.optBoolean("can_load_more") ?: false,
    )

    private fun project(json: JSONObject?): ProjectSnapshot = ProjectSnapshot(
        categories = json?.array("categories") { ProjectCategorySnapshot(it.optLong("id"), it.optString("name"), it.optInt("order"), it.optInt("visible")) } ?: emptyList(),
        articles = json?.array("articles") { projectArticle(it) } ?: emptyList(),
        selectedCategory = json?.nullableLong("selected_category"),
        isLoading = json?.optBoolean("is_loading") ?: false,
        errorMessage = json?.nullableString("error_message"),
        canLoadMore = json?.optBoolean("can_load_more") ?: false,
    )

    private fun collection(json: JSONObject?): CollectionSnapshot = CollectionSnapshot(
        articles = json?.array("articles") { article(it) } ?: emptyList(),
        isLoading = json?.optBoolean("is_loading") ?: false,
        errorMessage = json?.nullableString("error_message"),
        canLoadMore = json?.optBoolean("can_load_more", true) ?: true,
        pendingChange = json?.optBoolean("pending_change") ?: false,
    )

    private fun article(json: JSONObject) = ArticleSnapshot(
        id = json.optLong("id"), title = json.optString("title"), link = json.optString("link"),
        author = json.optString("author"), date = json.optString("nice_date"),
        collected = json.optBoolean("collect"), chapterName = json.optString("chapter_name"),
    )

    private fun banner(json: JSONObject) = BannerSnapshot(json.optLong("id"), json.optString("title"), json.optString("image_path"), json.optString("url"))

    private fun projectArticle(json: JSONObject) = ProjectArticleSnapshot(
        id = json.optLong("id"), title = json.optString("title"), link = json.optString("link"),
        author = json.optString("author"), date = json.optString("nice_date"),
        collected = json.optBoolean("collect"), chapterName = json.optString("chapter_name"),
        envelopePic = json.optString("envelope_pic"), thumbnail = json.optString("thumbnail"),
    )

    private fun JSONObject.nullableString(key: String): String? = if (isNull(key)) null else optString(key)
    private fun JSONObject.nullableLong(key: String): Long? = if (isNull(key)) null else optLong(key)
    private inline fun <T> JSONObject.array(key: String, mapper: (JSONObject) -> T): List<T> {
        val values = optJSONArray(key) ?: return emptyList()
        return buildList(values.length()) { for (index in 0 until values.length()) add(mapper(values.getJSONObject(index))) }
    }
}
