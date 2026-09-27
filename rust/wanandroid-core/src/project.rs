use crate::domain::Page;
use crate::repository::{RepositoryError, WanAndroidRepository};
use crate::session::SessionStore;
use crate::transport::HttpClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectCategory {
    pub id: u64,
    pub name: String,
    pub order: i32,
    pub visible: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectArticle {
    pub id: u64,
    pub title: String,
    pub link: String,
    pub author: String,
    pub nice_date: String,
    pub collect: bool,
    pub chapter_name: String,
    pub envelope_pic: String,
    pub thumbnail: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiProjectCategory {
    id: u64,
    name: String,
    #[serde(default)]
    order: i32,
    #[serde(default)]
    visible: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiProjectArticle {
    id: u64,
    title: String,
    link: String,
    #[serde(default)]
    author: String,
    #[serde(default)]
    nice_date: String,
    #[serde(default)]
    collect: bool,
    #[serde(default)]
    chapter_name: String,
    #[serde(default)]
    envelope_pic: String,
    #[serde(default)]
    thumbnail: String,
}

impl From<ApiProjectCategory> for ProjectCategory {
    fn from(value: ApiProjectCategory) -> Self {
        Self { id: value.id, name: value.name, order: value.order, visible: value.visible }
    }
}

impl From<ApiProjectArticle> for ProjectArticle {
    fn from(value: ApiProjectArticle) -> Self {
        Self {
            id: value.id,
            title: value.title,
            link: value.link,
            author: value.author,
            nice_date: value.nice_date,
            collect: value.collect,
            chapter_name: value.chapter_name,
            envelope_pic: value.envelope_pic,
            thumbnail: value.thumbnail,
        }
    }
}

pub trait ProjectRepository: Send + Sync {
    fn categories(&self) -> Result<Vec<ProjectCategory>, RepositoryError>;
    fn articles(&self, page: u32, category_id: u64) -> Result<Page<ProjectArticle>, RepositoryError>;
}

impl<C: HttpClient, S: SessionStore> ProjectRepository for WanAndroidRepository<C, S> {
    fn categories(&self) -> Result<Vec<ProjectCategory>, RepositoryError> {
        let response = self.get("/project/tree/json")?;
        let categories: Vec<ApiProjectCategory> = self.decode(&response.body)?;
        Ok(categories.into_iter().map(Into::into).collect())
    }

    fn articles(&self, page: u32, category_id: u64) -> Result<Page<ProjectArticle>, RepositoryError> {
        let response = self.get(format!("/project/list/{page}/json?cid={category_id}"))?;
        let page: crate::domain::ApiPage<ApiProjectArticle> = self.decode(&response.body)?;
        Ok(Page {
            cur_page: page.cur_page,
            datas: page.datas.into_iter().map(Into::into).collect(),
            over: page.over,
            page_count: page.page_count,
            total: page.total,
        })
    }
}
