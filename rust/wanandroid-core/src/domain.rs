use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Page<T> {
    pub cur_page: u32,
    pub datas: Vec<T>,
    pub over: bool,
    pub page_count: u32,
    pub total: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArticlePage {
    pub page: Page<Article>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Article {
    pub id: u64,
    pub title: String,
    pub link: String,
    pub author: String,
    pub nice_date: String,
    pub collect: bool,
    pub chapter_name: String,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Banner {
    pub id: u64,
    pub title: String,
    pub image_path: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ApiEnvelope<T> {
    pub error_code: i32,
    pub error_msg: String,
    pub data: T,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ApiPage<T> {
    pub cur_page: u32,
    pub datas: Vec<T>,
    pub over: bool,
    pub page_count: u32,
    pub total: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ApiArticle {
    pub id: u64,
    pub title: String,
    pub link: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub nice_date: String,
    #[serde(default)]
    pub collect: bool,
    #[serde(default)]
    pub chapter_name: String,
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ApiBanner {
    pub id: u64,
    pub title: String,
    pub image_path: String,
    pub url: String,
}

impl From<ApiPage<ApiArticle>> for ArticlePage {
    fn from(value: ApiPage<ApiArticle>) -> Self {
        Self {
            page: Page {
                cur_page: value.cur_page,
                over: value.over,
                page_count: value.page_count,
                total: value.total,
                datas: value.datas.into_iter().map(Into::into).collect(),
            },
        }
    }
}

impl From<ApiArticle> for Article {
    fn from(value: ApiArticle) -> Self {
        Self {
            id: value.id,
            title: value.title,
            link: value.link,
            author: value.author,
            nice_date: value.nice_date,
            collect: value.collect,
            chapter_name: value.chapter_name,
            tags: value.tags,
        }
    }
}

impl From<ApiBanner> for Banner {
    fn from(value: ApiBanner) -> Self {
        Self {
            id: value.id,
            title: value.title,
            image_path: value.image_path,
            url: value.url,
        }
    }
}
