# Telereads =  [Telegram](https://telegram.org/) + [Goodreads](https://www.goodreads.com/)

[![Telegram](http://trellobot.doomdns.org/telegrambadge.svg)](https://telegram.me/TelereadsBot)
[![Build Status](https://travis-ci.org/harababurel/telereads.svg?branch=master)](https://travis-ci.org/harababurel/telereads)
[![MIT License](http://img.shields.io/badge/license-MIT-blue.svg?style=flat)](https://github.com/harababurel/telereads/blob/master/LICENSE)

[@TelereadsBot](https://telegram.me/TelereadsBot) is an inline bot that allows you to search for books and quickly share them in a conversation.

## Demo

![Search by book title](https://thumbs.gfycat.com/HelpfulSparseKomododragon-size_restricted.gif)

![Search by ISBN](https://thumbs.gfycat.com/FlamboyantDistinctCaimanlizard-size_restricted.gif)

## Deploy your own

You can deploy your own instance of this bot. You will need a few things:

* A Telegram bot token. Contact [@BotFather](http://telegram.me/BotFather) in order to create a new bot and receive a token.
* A Goodreads API key. Apply for one [here](https://www.goodreads.com/api).
* A working installation of [Rust](https://www.rustup.rs/).

Once you have both of these keys, you need to store them as environment variables:

```bash
$ export TELEGRAM_TOKEN="123456789:XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
$ export GOODREADS_TOKEN="XXXXXXXXXXXXXXXXXXXX"
```

Finally, you can run the bot:

```bash
$ RUST_LOG=info cargo run --release
```

If this doesn't work, try the `nightly` branch of Rust.

## Contributing

Contributions are welcome. You can also help by reporting or fixing [bugs](https://github.com/harababurel/telereads/issues).
