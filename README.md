# Telereads =  [Telegram](https://telegram.org/) + [Goodreads](https://www.goodreads.com/)

[![Telegram](https://img.shields.io/badge/telegram-TelereadsBot-blue?logo=telegram&style=flat)](https://telegram.me/TelereadsBot)
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

And then install and run TelereadsBot:

### From GitHub

```bash
$ git clone https://github.com/harababurel/telereads.git
$ cd telereads
$ RUST_LOG=info cargo run --release
```

Aditionally, this repo includes a systemd service file ([telereads.service](https://github.com/harababurel/telereads/blob/master/telereads.service)) which can be used to deploy the bot. After setting the proper variables in the service file:

```bash
$ systemctl start telereads
```

### From crates.io

```bash
$ cargo install telereads
$ RUST_LOG=info telereads
```

## Contributing

Contributions are welcome. You can also help by reporting or fixing [bugs](https://github.com/harababurel/telereads/issues).

[![Star History Chart](https://api.star-history.com/svg?repos=harababurel/telereads&type=date&legend=top-left)](https://www.star-history.com/#harababurel/telereads&type=date&legend=top-left)
