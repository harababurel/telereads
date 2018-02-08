extern crate pretty_env_logger;
extern crate telereads;

use telereads::TelegramBot;

fn main() {
    pretty_env_logger::init();

    let telegram_token = std::env::var("TELEGRAM_TOKEN")
        .expect("Must provide telegram bot token as TELEGRAM_TOKEN environment variable.");
    let goodreads_token = std::env::var("GOODREADS_TOKEN")
        .expect("Must provide goodreads token as GOODREADS_TOKEN environment variable.");

    let mut bot = TelegramBot::with_tokens(&telegram_token, &goodreads_token);
    bot.run();
}
