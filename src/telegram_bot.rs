use reqwest;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json;
use models::{TelegramResult, SendMessageRequest, User, Update, Message, InlineQuery, AnswerInlineQuery, InlineQueryResult};
use goodreads::GoodreadsApi;

pub struct TelegramBot {
    token: String,
    goodreads: GoodreadsApi,
}

impl TelegramBot {
    pub fn with_tokens(telegram_token: &str, goodreads_token: &str) -> TelegramBot {
        TelegramBot { token: String::from(telegram_token), goodreads: GoodreadsApi::with_token(goodreads_token) }
    }

    fn get<T: DeserializeOwned>(&self, method: &str) -> Result<TelegramResult<T>, reqwest::Error> {
        let url = format!("https://api.telegram.org/bot{token}{method}",
                          token = self.token,
                          method = method);
        println!("GET-ing {}", &url);

        let mut response = reqwest::get(&url)?;
        let result: TelegramResult<T> = response.json()?;

        Ok(result)
    }

    fn post<P: Serialize, T: DeserializeOwned>(&self, method: &str, payload: &P) -> Result<TelegramResult<T>, reqwest::Error> {
        let url = format!("https://api.telegram.org/bot{token}{method}",
                          token = self.token,
                          method = method);
        println!("Payload is:\n{}", serde_json::to_string_pretty(payload).unwrap());

        let client = reqwest::Client::new();
        let mut resp = client.post(&url)
            .json(payload)
            .send()?;

        let result: TelegramResult<T> = resp.json()?;
        Ok(result)
    }

    pub fn get_me(&self) -> Result<TelegramResult<User>, reqwest::Error> {
        self.get("/getMe")
    }

    pub fn get_updates(&self, offset: Option<u32>) -> Result<TelegramResult<Vec<Update>>, reqwest::Error> {
        let mut method = String::from("/getUpdates");

        if offset.is_some() {
            method.push_str("?offset=");
            method.push_str(&offset.unwrap().to_string())
        }
        self.get(&method)
    }

    pub fn answer_inline_query(&self, query: InlineQuery) -> Result<bool, reqwest::Error> {
        match self.goodreads.get_books(&query.query) {
            Ok(works) => {
                let results = works.iter().map(|work| {
                    InlineQueryResult::from(work)
                }).collect();

                let answer = AnswerInlineQuery {
                    inline_query_id: query.id,
                    results: results,
                    ..Default::default()
                };

                match self.post("/answerInlineQuery", &answer): Result<TelegramResult<bool>, reqwest::Error> {
                    Ok(result) => Ok(result.get()),
                    Err(e) => Err(e),
                }
            }
            Err(e) => {
                println!("Goodreads error: {:?}", e);
                Ok(false)
            }
        }
    }

    pub fn send_message(&self, chat_id: u64, text: &str) -> Result<Message, reqwest::Error> {
        let request = SendMessageRequest {
            chat_id: chat_id,
            text: String::from(text),
            ..Default::default()
        };

        match self.post("/sendMessage", &request): Result<TelegramResult<Message>, reqwest::Error> {
            Ok(result) => Ok(result.get()),
            Err(e) => Err(e)
        }
    }
}
