use super::{Location, User};
use ammonia::clean;
use ammonia::url::Url;
use goodreads::Work;
use number_prefix::{decimal_prefix, Prefixed, Standalone};
use std::convert::From;

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    pub query: String,
    pub offset: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AnswerInlineQuery {
    pub inline_query_id: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub results: Vec<InlineQueryResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_personal: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pm_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pm_parameter: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum InlineQueryResult {
    InlineQueryResultCachedAudio,
    InlineQueryResultCachedDocument,
    InlineQueryResultCachedGif,
    InlineQueryResultCachedMpeg4Gif,
    InlineQueryResultCachedPhoto,
    InlineQueryResultCachedSticker,
    InlineQueryResultCachedVideo,
    InlineQueryResultCachedVoice,
    InlineQueryResultArticle {
        #[serde(rename = "type")]
        _type: String,
        id: String,
        title: String,
        input_message_content: InputMessageContent,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<InlineKeyboardMarkup>,
        #[serde(skip_serializing_if = "Option::is_none")]
        url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        hide_url: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_width: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_height: Option<i64>,
    },
    InlineQueryResultAudio,
    InlineQueryResultContact,
    InlineQueryResultGame,
    InlineQueryResultDocument,
    InlineQueryResultGif,
    InlineQueryResultLocation,
    InlineQueryResultMpeg4Gif,
    InlineQueryResultPhoto,
    InlineQueryResultVenue,
    InlineQueryResultVideo,
    InlineQueryResultVoice,
}

impl<'a> From<&'a Work> for InlineQueryResult {
    fn from(work: &'a Work) -> Self {
        let book_url = Url::parse_with_params(
            "https://www.goodreads.com/book/title",
            &[("id", &work.best_book.title)],
        ).unwrap_or_else(|_| Url::parse("https://www.goodreads.com").unwrap());

        let author_url = Url::parse(&format!(
            "https://www.goodreads.com/book/author/{}",
            &work.best_book.author.name
        )).unwrap_or_else(|_| Url::parse("https://www.goodreads.com").unwrap());

        let rating = match work.average_rating {
            Some(value) => format!("{:.1}", value),
            None => String::from("?"),
        };

        let rating_count = match decimal_prefix(work.ratings_count as f32) {
            Standalone(value) => format!("{}", value),
            Prefixed(prefix, n) => format!("{:.0}{}", n, prefix),
        };

        let message_text = clean(&format!(
            "<a href=\"{book_url}\">{title}</a>\nAuthor: \
             <a href=\"{author_url}\">{author}</a>\nAvg. {rating}/5 ({rating_count} ratings)",
            book_url = book_url,
            author_url = author_url,
            title = &work.best_book.title,
            author = &work.best_book.author.name,
            rating = rating,
            rating_count = rating_count
        ));

        let description = format!(
            "{author}\n{rating}/5 ({rating_count} ratings)",
            author = work.best_book.author.name,
            rating = rating,
            rating_count = rating_count
        );

        InlineQueryResult::InlineQueryResultArticle {
            _type: String::from("article"),
            id: work.id.to_string(),
            title: work.best_book.title.clone(),
            input_message_content: InputMessageContent::InputTextMessageContent {
                message_text,
                parse_mode: Some(String::from("HTML")),
                disable_web_page_preview: None,
            },
            reply_markup: None,
            url: Some(book_url.to_string()),
            hide_url: Some(true),
            description: Some(description),
            thumb_url: Some(work.best_book.small_image_url.clone()),
            thumb_width: None,
            thumb_height: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChosenInlineQuery {}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallbackQuery {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShippingQuery {}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreCheckoutQuery {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum InputMessageContent {
    InputTextMessageContent {
        message_text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        disable_web_page_preview: Option<bool>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineKeyboardMarkup {}
