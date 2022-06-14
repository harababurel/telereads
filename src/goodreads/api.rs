use super::{GoodreadsResponse, Work};
use std::convert::From;

#[derive(Debug)]
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

        let resp = retry::retry(retry::delay::Fixed::from_millis(100).take(3), || {
            reqwest::blocking::get(url.to_owned())
        })?;

        let text: String = resp.text()?;
        let goodreads_response: GoodreadsResponse = serde_xml_rs::from_str(&text)?;

        Ok(goodreads_response)
    }

    pub fn get_books(&self, query: &str) -> Result<Vec<Work>, GoodreadsError> {
        if query.is_empty() {
            return Ok(Vec::new());
        }

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
    ParseError(ammonia::url::ParseError),
    SerdeXmlError(serde_xml_rs::Error),
    RetryError(retry::Error<reqwest::Error>),
}

impl From<reqwest::Error> for GoodreadsError {
    fn from(e: reqwest::Error) -> Self {
        GoodreadsError::ReqwestError(e)
    }
}

impl From<ammonia::url::ParseError> for GoodreadsError {
    fn from(e: ammonia::url::ParseError) -> Self {
        GoodreadsError::ParseError(e)
    }
}

impl From<serde_xml_rs::Error> for GoodreadsError {
    fn from(e: serde_xml_rs::Error) -> Self {
        GoodreadsError::SerdeXmlError(e)
    }
}

impl From<retry::Error<reqwest::Error>> for GoodreadsError {
    fn from(e: retry::Error<reqwest::Error>) -> Self {
        GoodreadsError::RetryError(e)
    }
}
