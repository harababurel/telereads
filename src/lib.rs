#![feature(type_ascription)]
extern crate reqwest;
extern crate retry;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;
extern crate number_prefix;

mod goodreads;
mod telegram;

pub use telegram::TelegramBot;
