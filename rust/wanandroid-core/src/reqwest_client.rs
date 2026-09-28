use crate::transport::{HttpClient, HttpRequest, HttpResponse};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReqwestClientError {
    #[error("invalid base URL: {0}")]
    InvalidBaseUrl(String),
    #[error("invalid request header {name}: {message}")]
    InvalidHeader { name: String, message: String },
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("response body failed: {0}")]
    ResponseBody(#[source] reqwest::Error),
}

/// Production blocking transport used at the native boundary.
///
/// The core remains platform-neutral: Android/iOS can inject this client or provide another
/// `HttpClient`. Blocking execution is deliberately isolated here; callers should run effects
/// off the UI thread in the platform adapter.
pub struct ReqwestHttpClient {
    base_url: String,
    client: Client,
}

impl ReqwestHttpClient {
    pub fn new(base_url: impl Into<String>, timeout: Duration) -> Result<Self, ReqwestClientError> {
        let base_url = base_url.into().trim_end_matches('/').to_owned();
        reqwest::Url::parse(&base_url)
            .map_err(|error| ReqwestClientError::InvalidBaseUrl(error.to_string()))?;
        let client = Client::builder().timeout(timeout).build()?;
        Ok(Self { base_url, client })
    }

    pub fn wanandroid(timeout: Duration) -> Result<Self, ReqwestClientError> {
        Self::new("https://www.wanandroid.com", timeout)
    }

    fn headers(
        &self,
        source: &std::collections::BTreeMap<String, String>,
    ) -> Result<HeaderMap, ReqwestClientError> {
        let mut headers = HeaderMap::new();
        for (name, value) in source {
            let header_name = HeaderName::from_bytes(name.as_bytes()).map_err(|error| {
                ReqwestClientError::InvalidHeader {
                    name: name.clone(),
                    message: error.to_string(),
                }
            })?;
            let header_value = HeaderValue::from_str(value).map_err(|error| {
                ReqwestClientError::InvalidHeader {
                    name: name.clone(),
                    message: error.to_string(),
                }
            })?;
            headers.insert(header_name, header_value);
        }
        Ok(headers)
    }
}

impl HttpClient for ReqwestHttpClient {
    fn execute(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let url = format!("{}{}", self.base_url, request.path);
        let headers = self
            .headers(&request.headers)
            .map_err(|error| error.to_string())?;
        let response = match request.method {
            crate::transport::HttpMethod::Get => self
                .client
                .get(url)
                .headers(headers)
                .query(&request.form)
                .send(),
            crate::transport::HttpMethod::Post => self
                .client
                .post(url)
                .headers(headers)
                .form(&request.form)
                .send(),
        }
        .map_err(|error| error.to_string())?;

        let response_headers = response
            .headers()
            .iter()
            .filter_map(|(name, value)| {
                value
                    .to_str()
                    .ok()
                    .map(|value| (name.to_string(), value.to_owned()))
            })
            .collect();
        let body = response
            .text()
            .map_err(|error| ReqwestClientError::ResponseBody(error).to_string())?;
        Ok(HttpResponse {
            body,
            headers: response_headers,
        })
    }
}
