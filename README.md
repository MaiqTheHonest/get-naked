This program scrapes the main page of `nakedcapitalism.com` for article links and sends them to a Telegram channel / user via bot with my custom [Instant View](https://instantview.telegram.org) template.


## Usage:

If you want to **just read** articles from naked capitalism on Telegram, you can join my public Telegram channel where they are posted every day: https://t.me/+Qkp6stSOGMRkZTJk

<br/>

  <img
      src="https://github.com/user-attachments/assets/8f0a0299-c664-4ad4-9572-ee9361b4976a" 
      width=30%
      title="My Image"
      alt="My Image"
  />



___

If you want to use this program as a template for your own Rust NC reposter, you will need to download the `release` folder and customize these in `.env`:

1. Your Bot Token from Telegram's [@BotFather](https://telegram.me/BotFather).
2. Your Chat Id or the Channel Id where you want the articles to be reposted in.

and then use any task scheduler to run `get_naked.exe` in that folder.

___

Obviously you can also download the repo and change my custom Instant View rhash in `bot.rs` or anything else, and then `cargo build`. Just don't forget to encode the urls, as otherwise IV will not work (Telegram sanitizes rhash).
