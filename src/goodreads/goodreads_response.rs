use serde::de::{self, Deserialize, Deserializer};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Deserialize, Debug)]
pub struct GoodreadsResponse {
    pub search: Search,
}

#[derive(Deserialize, Debug)]
pub struct Search {
    pub query: String,

    #[serde(rename = "results-start")]
    #[serde(deserialize_with = "from_str")]
    pub results_start: u32,

    #[serde(rename = "results-end")]
    #[serde(deserialize_with = "from_str")]
    pub results_end: u32,

    #[serde(rename = "total-results")]
    #[serde(deserialize_with = "from_str")]
    pub total_results: u32,

    pub source: String,

    #[serde(rename = "query-time-seconds")]
    #[serde(deserialize_with = "from_str")]
    pub query_time_seconds: f32,

    #[serde(rename = "results")]
    pub results_container: ResultsContainer,
}

#[derive(Deserialize, Debug)]
pub struct ResultsContainer {
    #[serde(rename = "work")]
    pub results: Vec<Work>,
}


#[derive(Deserialize, Debug, Default)]
pub struct Work {
    #[serde(deserialize_with = "from_str")]
    pub id: u32,

    #[serde(deserialize_with = "from_str")]
    pub books_count: u32,

    #[serde(deserialize_with = "from_str")]
    pub ratings_count: u32,

    #[serde(deserialize_with = "from_str")]
    pub text_reviews_count: u32,

    #[serde(deserialize_with = "from_str_option")]
    pub original_publication_year: Option<i32>,

    #[serde(deserialize_with = "from_str_option")]
    pub original_publication_month: Option<u32>,

    #[serde(deserialize_with = "from_str_option")]
    pub original_publication_day: Option<u32>,

    #[serde(deserialize_with = "from_str_option")]
    pub average_rating: Option<f32>,

    pub best_book: Book,
}


#[derive(Deserialize, Debug, Default)]
pub struct Book {
    #[serde(deserialize_with = "from_str")]
    pub id: u32,
    pub title: String,
    pub author: Author,
    pub image_url: String,
    pub small_image_url: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct Author {
    #[serde(deserialize_with = "from_str")]
    pub id: u32,
    pub name: String,
}

// https://github.com/serde-rs/json/issues/317
fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;

    T::from_str(&s).map_err(de::Error::custom)
}

fn from_str_option<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        return Ok(None);
    }

    match T::from_str(&s).map_err(de::Error::custom) {
        Ok(x) => Ok(Some(x)),
        Err(e) => Err(e),
    }
}
