use super::{User, Location};

#[derive(Serialize, Deserialize, Debug)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    pub query: String,
    pub offset: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnswerInlineQuery {
    pub inline_query_id: String,
    pub results: Vec<InlineQueryResult>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<u32>,

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
        thumb_width: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumb_height: Option<u32>,
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
