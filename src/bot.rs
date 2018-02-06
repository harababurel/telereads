
use reqwest;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json;
use models::{TelegramResult, User, Update, InlineQuery, AnswerInlineQuery, InlineQueryResult, InputMessageContent};


pub struct Bot {}

impl Bot {
    fn get<T: DeserializeOwned>(method: &str) -> Result<TelegramResult<T>, reqwest::Error> {
        let mut url = String::from("https://api.telegram.org/bot536722142:AAFTsVGNVt6vVqFFeF49vjO3drfXxG64m-8");
        url.push_str(&method);

        let mut resp = reqwest::get(&url)?;
        let result: TelegramResult<T> = resp.json()?;

        Ok(result)
    }

    fn post<P: Serialize, T: DeserializeOwned>(method: &str, payload: &P) -> Result<TelegramResult<T>, reqwest::Error> {
        let mut url = String::from("https://api.telegram.org/bot536722142:AAFTsVGNVt6vVqFFeF49vjO3drfXxG64m-8");
        url.push_str(&method);

        println!("payload is:\n{}", serde_json::to_string_pretty(payload).unwrap());

        let client = reqwest::Client::new();
        let mut resp = client.post(&url)
            .json(payload)
            .send()?;
        let result: TelegramResult<T> = resp.json()?;

//        println!("response status: {}", &resp.status());
//        let text = &resp.text().unwrap();
//        println!("response text:\n{}", serde_json::to_string_pretty(text).unwrap());

        Ok(result)
    }

    pub fn get_me() -> Result<TelegramResult<User>, reqwest::Error> {
        Bot::get("/getMe")
    }

    pub fn get_updates() -> Result<TelegramResult<Vec<Update>>, reqwest::Error> {
        Bot::get("/getUpdates")
    }

    pub fn answer_inline_query(query: InlineQuery) -> bool {
        let result = InlineQueryResult::InlineQueryResultArticle {
            _type: String::from("article"),
            id: String::from("1234"),
            title: String::from("Some article"),
            input_message_content: InputMessageContent::InputTextMessageContent {
                message_text: String::from("some message text"),
                parse_mode: None,
                disable_web_page_preview: None,
            },
            reply_markup: None,
            url: Some(String::from("https://www.goodreads.com/book/show/414999.Childhood_s_End")),
            hide_url: None,
            description: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        };

        let answer = AnswerInlineQuery {
            inline_query_id: query.id,
            results: vec![result],
            cache_time: None,
            is_personal: None,
            next_offset: None,
            switch_pm_text: None,
            switch_pm_parameter: None,
        };

        match Bot::post("/answerInlineQuery", &answer): Result<TelegramResult<bool>, reqwest::Error> {
            Ok(result) => {
                result.get()
            }
            Err(e) => {
                println!("Error answering inline query: {:?}", e);
                false
            }
        }
    }
}
