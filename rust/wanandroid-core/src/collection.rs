use crate::domain::Article;
use crate::repository::{RepositoryError, WanAndroidRepository};
use crate::session::SessionStore;
use crate::transport::HttpClient;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CollectionPage {
    pub cur_page: u32,
    pub datas: Vec<Article>,
    pub over: bool,
    pub page_count: u32,
    pub total: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiCollectionPage {
    cur_page: u32,
    datas: Vec<crate::domain::ApiArticle>,
    over: bool,
    page_count: u32,
    total: u32,
}

impl From<ApiCollectionPage> for CollectionPage {
    fn from(value: ApiCollectionPage) -> Self {
        Self {
            cur_page: value.cur_page,
            datas: value.datas.into_iter().map(Into::into).collect(),
            over: value.over,
            page_count: value.page_count,
            total: value.total,
        }
    }
}

pub trait CollectionRepository: Send + Sync {
    fn list(&self, page: u32) -> Result<CollectionPage, RepositoryError>;
    fn collect(&self, article_id: u64) -> Result<(), RepositoryError>;
    fn uncollect(&self, origin_id: u64) -> Result<(), RepositoryError>;
}

impl<C: HttpClient, S: SessionStore> CollectionRepository for WanAndroidRepository<C, S> {
    fn list(&self, page: u32) -> Result<CollectionPage, RepositoryError> {
        let response = self.get(format!("/lg/collect/list/{page}/json"))?;
        let page: ApiCollectionPage = self.decode(&response.body)?;
        Ok(page.into())
    }

    fn collect(&self, article_id: u64) -> Result<(), RepositoryError> {
        let response = self.post_form(
            format!("/lg/collect/{article_id}/json"),
            std::collections::BTreeMap::new(),
        )?;
        let _: serde_json::Value = self.decode(&response.body)?;
        Ok(())
    }

    fn uncollect(&self, origin_id: u64) -> Result<(), RepositoryError> {
        let response = self.post_form(
            format!("/lg/uncollect_originId/{origin_id}/json"),
            std::collections::BTreeMap::new(),
        )?;
        let _: serde_json::Value = self.decode(&response.body)?;
        Ok(())
    }
}
