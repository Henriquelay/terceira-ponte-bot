//! A simple bot that replies with an image to an important road where I live

#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]
#![warn(unused_crate_dependencies)]
#![deny(missing_docs)]
#![deny(missing_doc_code_examples)]

use futures::future::join_all;
use scraper::{Html, Selector};
use teloxide::{prelude::*, types::InputFile, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "Estes comandos são suportados:"
)]
enum Command {
    #[command(description = "mostra esta ajuda.")]
    Help,
    #[command(description = "mostra esta ajuda.")]
    Ajuda,
    #[command(description = "mostra fotos da ponte agora.")]
    Ponte,
}

async fn load_rodosol() -> ResponseResult<String> {
    const URL: &str = "https://www.rodosol.com.br/de-olho-na-via/";
    let resp = reqwest::get(URL).await?;
    Ok(resp.text().await?)
}

fn parse_images_url(html: &str) -> Vec<String> {
    const IMAGE_IDENTIFYING_PART: &str = "https://www.rodosol.com.br/_util/cameras/camera";
    let fragment = Html::parse_fragment(html);
    let selector = Selector::parse("img").unwrap();
    let mut res: Vec<String> = fragment
        .select(&selector)
        .filter_map(|element| {
            let src = element.value().attr("src")?;
            if src.contains(IMAGE_IDENTIFYING_PART) {
                Some(src.to_string())
            } else {
                None
            }
        })
        .collect();
    res.dedup();
    dbg!(&res);
    res
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help | Command::Ajuda => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .allow_sending_without_reply(false)
                .reply_to_message_id(msg.id)
                .await?
        }
        Command::Ponte => {
            let website = load_rodosol();
            let _ = bot
                .send_message(msg.chat.id, "Aguarde...")
                .allow_sending_without_reply(false)
                .reply_to_message_id(msg.id);
            let images_url = parse_images_url(&website.await?);
            let mut messages_futures = vec![];
            for url in images_url {
                let url = reqwest::Url::parse(&url).unwrap();
                let msg = bot
                    .send_photo(msg.chat.id, InputFile::url(url))
                    .allow_sending_without_reply(false)
                    .reply_to_message_id(msg.id).await; // TODO Don't await here
                messages_futures.push(msg);
            }
            // join_all(messages_futures).await;
            todo!()
        }
    };

    Ok(())
}
