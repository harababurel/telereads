use super::{GoodreadsResponse, Work};
use reqwest;
use retry;
use serde_xml_rs;
use std::convert::From;

pub struct GoodreadsApi {
    token: String,
}

impl GoodreadsApi {
    pub fn with_token(token: &str) -> GoodreadsApi {
        GoodreadsApi {
            token: String::from(token),
        }
    }

    fn get(&self, url: &str, params: &[(&str, &str)]) -> Result<GoodreadsResponse, GoodreadsError> {
        let url = reqwest::Url::parse_with_params(url, params)?;
        info!("GET-ing {}", &url);

        let mut resp = retry::retry(
            3,
            100,
            || reqwest::get(url.to_owned()),
            |result| result.is_ok(),
        )??;

        let text: String = resp.text()?;
        let goodreads_response: GoodreadsResponse = serde_xml_rs::deserialize(text.as_bytes())?;

        Ok(goodreads_response)
    }

    pub fn get_books(&self, query: &str) -> Result<Vec<Work>, GoodreadsError> {
        let goodreads_response = self.get(
            "https://www.goodreads.com/search/index.xml",
            &[("key", &self.token), ("q", query)],
        )?;

        Ok(goodreads_response.search.results_container.results)
    }
}

#[derive(Debug)]
pub enum GoodreadsError {
    ReqwestError(reqwest::Error),
    UrlError(reqwest::UrlError),
    SerdeXmlError(serde_xml_rs::Error),
    RetryError(retry::RetryError),
}

impl From<reqwest::Error> for GoodreadsError {
    fn from(e: reqwest::Error) -> Self {
        GoodreadsError::ReqwestError(e)
    }
}

impl From<reqwest::UrlError> for GoodreadsError {
    fn from(e: reqwest::UrlError) -> Self {
        GoodreadsError::UrlError(e)
    }
}

impl From<serde_xml_rs::Error> for GoodreadsError {
    fn from(e: serde_xml_rs::Error) -> Self {
        GoodreadsError::SerdeXmlError(e)
    }
}

impl From<retry::RetryError> for GoodreadsError {
    fn from(e: retry::RetryError) -> Self {
        GoodreadsError::RetryError(e)
    }
}
