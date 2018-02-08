#![feature(type_ascription)]
extern crate ammonia;
extern crate reqwest;
extern crate retry;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;
extern crate number_prefix;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod goodreads;
mod telegram;

pub use telegram::TelegramBot;
