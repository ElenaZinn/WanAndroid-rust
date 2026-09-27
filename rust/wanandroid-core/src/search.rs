use crate::domain::{ApiArticle, ApiPage, ArticlePage};
use crate::repository::{RepositoryError, WanAndroidRepository};
use crate::session::SessionStore;
use crate::transport::HttpClient;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HotKey {
    pub id: u64,
    pub name: String,
    pub link: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ApiHotKey {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub link: String,
}

impl From<ApiHotKey> for HotKey {
    fn from(value: ApiHotKey) -> Self {
        Self {
            id: value.id,
            name: value.name,
            link: value.link,
        }
    }
}

pub trait SearchRepository: Send + Sync {
    fn hot_keys(&self) -> Result<Vec<HotKey>, RepositoryError>;
    fn search(&self, page: u32, keyword: &str) -> Result<ArticlePage, RepositoryError>;
    fn search_author(&self, page: u32, author: &str) -> Result<ArticlePage, RepositoryError>;
}

impl<C: HttpClient, S: SessionStore> SearchRepository for WanAndroidRepository<C, S> {
    fn hot_keys(&self) -> Result<Vec<HotKey>, RepositoryError> {
        let response = self.get("/hotkey/json")?;
        let hot_keys: Vec<ApiHotKey> = self.decode(&response.body)?;
        Ok(hot_keys.into_iter().map(Into::into).collect())
    }

    fn search(&self, page: u32, keyword: &str) -> Result<ArticlePage, RepositoryError> {
        let form = BTreeMap::from([(String::from("k"), keyword.to_owned())]);
        let response = self.post_form(format!("/article/query/{page}/json"), form)?;
        let page: ApiPage<ApiArticle> = self.decode(&response.body)?;
        Ok(page.into())
    }

    fn search_author(&self, page: u32, author: &str) -> Result<ArticlePage, RepositoryError> {
        let query = format!("/article/list/{page}/json?author={}", encode_query(author));
        let response = self.get(&query)?;
        let page: ApiPage<ApiArticle> = self.decode(&response.body)?;
        Ok(page.into())
    }
}

fn encode_query(value: &str) -> String {
    value
        .bytes()
        .map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (byte as char).to_string()
            }
            _ => format!("%{byte:02X}"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::encode_query;

    #[test]
    fn query_encoding_is_deterministic_and_does_not_leak_spaces() {
        assert_eq!(encode_query("rust android"), "rust%20android");
        assert_eq!(encode_query("C++"), "C%2B%2B");
    }
}
