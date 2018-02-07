#![feature(type_ascription)]
extern crate telereads;
extern crate reqwest;
extern crate serde_json;
extern crate serde_xml_rs;

use telereads::telegram_bot::TelegramBot;
use telereads::models::{TelegramResult, Update};

fn main() {
    let telegram_token = std::env::var("TELEGRAM_TOKEN")
        .expect("Must provide telegram bot token as TELEGRAM_TOKEN environment variable.");
    let goodreads_token = std::env::var("GOODREADS_TOKEN")
        .expect("Must provide goodreads token as GOODREADS_TOKEN environment variable.");

    let bot = TelegramBot::with_tokens(&telegram_token, &goodreads_token);

    let mut offset: Option<u32> = None;
    loop {
        match bot.get_updates(offset): Result<TelegramResult<Vec<Update>>, reqwest::Error> {
            Ok(result) => {
                println!("Found result: {:?}", result);

                for update in result.get() {
                    println!("Found update:\n{}", serde_json::to_string_pretty(&update).unwrap());

                    if offset.is_none() {
                        offset = Some(update.update_id + 1);
                    } else {
                        offset = Some(std::cmp::max(offset.unwrap(), update.update_id) + 1);
                    }

                    println!("Offset is {:?}", offset);

                    if update.inline_query.is_some() {
                        update.inline_query.map(|query| {
                            print!("Answering inline query...");
                            match bot.answer_inline_query(query) {
                                Ok(success) => println!(" Answer sent! Success = {}", success),
                                Err(e) => println!(" Could not answer inline query: {:#?}", e),
                            };
                        });
                    }

                    if update.message.is_some() {
                        update.message.map(|message| {
                            print!("Answering message...");
                            match bot.send_message(message.chat.id, "unrecognized command") {
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
