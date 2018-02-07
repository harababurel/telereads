use reqwest;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json;
use models::{TelegramResult, SendMessageRequest, User, Update, Message, InlineQuery, AnswerInlineQuery, InlineQueryResult};
use goodreads::GoodreadsApi;
use std;

pub struct TelegramBot {
    token: String,
    goodreads: GoodreadsApi,
    client: reqwest::Client,
    offset: Option<u64>,
}

impl TelegramBot {
    pub fn with_tokens(telegram_token: &str, goodreads_token: &str) -> TelegramBot {
        TelegramBot {
            token: String::from(telegram_token),
            goodreads: GoodreadsApi::with_token(goodreads_token),
            client: reqwest::Client::new(),
            offset: None,
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.get_updates(): Result<TelegramResult<Vec<Update>>, reqwest::Error> {
                Ok(result) => {
                    println!("Found result: {:?}", result);

                    for update in result.unwrap() {
                        println!("Found update:\n{}", serde_json::to_string_pretty(&update).unwrap());

                        if update.inline_query.is_some() {
                            update.inline_query.map(|query| {
                                print!("Answering inline query...");
                                match self.answer_inline_query(query) {
                                    Ok(success) => println!(" Answer sent! Success = {}", success),
                                    Err(e) => println!(" Could not answer inline query: {:#?}", e),
                                };
                            });
                        }

                        if update.message.is_some() {
                            update.message.map(|message| {
                                print!("Answering message...");
                                match self.send_message(message.chat.id, "unrecognized command") {
                                    Ok(message) => println!("OK! Message: {:#?}", message),
                                    Err(e) => println!("Could not answer message: {:#?}", e),
                                };
                            });
                        }
                    }
                }
                Err(e) => println!("Something bad: {:?}", e),
            };

            std::thread::sleep(std::time::Duration::new(1, 0));
        }
    }

    fn get<T>(&self, method: &str) -> Result<TelegramResult<T>, reqwest::Error>
        where T: DeserializeOwned {
        let url = format!("https://api.telegram.org/bot{token}{method}",
                          token = self.token,
                          method = method);
        println!("GET-ing {}", &url);

        let mut response = reqwest::get(&url)?;
        let result: TelegramResult<T> = response.json()?;

        Ok(result)
    }

    fn post<P, T>(&self, method: &str, payload: &P) -> Result<TelegramResult<T>, reqwest::Error>
        where P: Serialize,
              T: DeserializeOwned {
        Ok(self.client.post(&format!("https://api.telegram.org/bot{}{}", self.token, method))
            .json(payload)
            .send()?
            .json()?)
    }

    pub fn get_me(&self) -> Result<TelegramResult<User>, reqwest::Error> {
        self.get("/getMe")
    }

    pub fn get_updates(&mut self) -> Result<TelegramResult<Vec<Update>>, reqwest::Error> {
        let method = format!("/getUpdates{maybe_offset}",
                             maybe_offset = match self.offset {
                                 Some(val) => format!("?offset={}", val),
                                 None => String::new(),
                             });

        let result: Result<TelegramResult<Vec<Update>>, reqwest::Error> = self.get(&method);
        if result.is_ok() {
            result.as_ref().unwrap().get_ref().into_iter().for_each(|update| {
                self.offset = match self.offset {
                    Some(val) => Some(std::cmp::max(val, update.update_id) + 1),
                    None => Some(update.update_id + 1),
                }
            });
        }

        result
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

                match self.post("/answerInlineQuery", &answer) {
                    Ok(result) => Ok(result.unwrap()),
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
            chat_id,
            text: String::from(text),
            ..Default::default()
        };

        match self.post("/sendMessage", &request) {
            Ok(result) => Ok(result.unwrap()),
            Err(e) => Err(e)
        }
    }
}
