#![feature(type_ascription)]
extern crate telereads;
extern crate reqwest;
extern crate serde_json;

use telereads::bot::Bot;
use telereads::models::{TelegramResult, Update};

fn main() {
    match Bot::get_updates(): Result<TelegramResult<Vec<Update>>, reqwest::Error> {
        Ok(result) => {
            println!("Found result: {:?}", result);

            for update in result.get() {
                println!("Found update:\n{}", serde_json::to_string_pretty(&update).unwrap());
                update.inline_query.map(|query| {
                    print!("Answering inline query...");
                    if Bot::answer_inline_query(query) {
                        println!(" OK!");
                    } else {
                        println!(" Could not answer query!");
                    }
                });
            }
        }
        Err(e) => println!("Something bad: {:?}", e),
    };
}
