use goodreads::GoodreadsApi;
use reqwest;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use std;
use std::fmt::Debug;
use telegram::models::{AnswerInlineQuery, InlineQuery, InlineQueryResult, Message,
                       SendMessageRequest, Update, User};
use telegram::TelegramResult;

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
            match self.get_updates() {
                Ok(result) => {
                    if !result.ok() {
                        continue;
                    }

                    for update in result.unwrap() {
                        update.inline_query.map(|query| {
                            if !&query.query.is_empty() {
                                info!(
                                    "Received inline query from {user}: \"{message}\"",
                                    user =
                                        &query.from.username.as_ref().unwrap_or(&"???".to_string()),
                                    message = &query.query
                                );
                            }

                            match self.answer_inline_query(query) {
                                Ok(success) => info!("Answer sent! Success = {}", success),
                                Err(e) => error!("Could not answer inline query: {:#?}", e),
                            };
                        });

                        update.message.map(|message| {
                            match self.send_message(message.chat.id, "@TelereadsBot allows you to search for books on Goodreads and quickly send them to your chat partner.\n\nJust type @TelereadsBot in any chat, followed by a query (<i>i.e.</i> book title, ISBN, or author name), without pressing 'send'. You can choose any result from the pop-up window that will show up and send it by simply clicking on it.\n\nFor instance, try typing <code>@TelereadsBot lord of the rings</code> in this chat, and wait for the results to appear.") {
                                    Ok(_) => info!("Message successfully sent."),
                                    Err(e) => error!("Could not send message: {:#?}", e),
                                };
                        });
                    }
                }
                Err(e) => error!("Could not get updates: {}", e),
            };
        }
    }

    fn get<T>(&self, method: &str) -> Result<TelegramResult<T>, reqwest::Error>
    where
        T: DeserializeOwned + Debug + Default,
    {
        let url = format!("https://api.telegram.org/bot{}{}", self.token, method);

        Ok(self.client
            .get(&url)
            .query(&[("timeout", 20)])
            .send()?
            .json()?)
    }

    fn post<P, T>(&self, method: &str, payload: &P) -> Result<TelegramResult<T>, reqwest::Error>
    where
        P: Serialize + Debug,
        T: DeserializeOwned + Debug + Default,
    {
        let url = format!("https://api.telegram.org/bot{}{}", self.token, method);
        debug!("POST-ing {}", &url);
        debug!("Payload:\n{:#?}", &payload);

        Ok(self.client.post(&url).json(payload).send()?.json()?)
    }

    pub fn get_me(&self) -> Result<TelegramResult<User>, reqwest::Error> {
        self.get("/getMe")
    }

    pub fn get_updates(&mut self) -> Result<TelegramResult<Vec<Update>>, reqwest::Error> {
        let method = format!(
            "/getUpdates{maybe_offset}",
            maybe_offset = match self.offset {
                Some(val) => format!("?offset={}", val),
                None => String::new(),
            }
        );

        let result: Result<TelegramResult<Vec<Update>>, reqwest::Error> = self.get(&method);
        if result.is_ok() {
            result
                .as_ref()
                .unwrap()
                .get_ref()
                .into_iter()
                .for_each(|update| {
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
                info!("Received {} books from Goodreads", works.len());
                let results = works.iter().map(InlineQueryResult::from).collect();

                let answer = AnswerInlineQuery {
                    inline_query_id: query.id,
                    results,
                    ..Default::default()
                };

                match self.post("/answerInlineQuery", &answer):
                    Result<TelegramResult<bool>, reqwest::Error>
                {
                    Ok(result) => {
                        debug!(
                            "Received a response from /answerInlineQuery: ok = {:#?}",
                            &result.ok()
                        );
                        if result.has_description() {
                            debug!("description = {}", &result.description.as_ref().unwrap());
                        }
                        Ok(result.ok())
                    }
                    Err(e) => {
                        error!(
                            "Did not receive a response from /answerInlineQuery. Error:\n{:#?}",
                            &e
                        );
                        Err(e)
                    }
                }
            }
            Err(e) => {
                error!("Goodreads error: {:?}", e);
                Ok(false)
            }
        }
    }

    pub fn send_message(
        &self,
        chat_id: u64,
        text: &str,
    ) -> Result<TelegramResult<Message>, reqwest::Error> {
        let request = SendMessageRequest {
            chat_id,
            text: String::from(text),
            parse_mode: Some(String::from("HTML")),
            ..Default::default()
        };

        self.post("/sendMessage", &request)
    }
}
