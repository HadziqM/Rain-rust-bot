#![allow(dead_code)]

pub mod commands;
mod event;
pub mod material;
pub mod reusable;

use event::Handler;
use event::{
    interaction::{ComponentBundle, ModalBundle, Mybundle, SlashBundle},
    message::MsgBundle,
};
use lazy_static::lazy_static;
use material::ItemPedia;
use reusable::component::bounty::BountySubmit;
use reusable::component::discord::AppReg;
use reusable::component::error::ErrorLog;
use reusable::component::{registered::Reg, Components, MyErr, Mytrait};
use reusable::config::Init;
use reusable::gpt::chat::CompModel;
use reusable::image_edit::Images;
use reusable::postgress::PgConn;
use serenity::prelude::{Mutex, *};
use std::collections::HashMap;

lazy_static! {
    static ref COOLDOWN: Mutex<HashMap<String, i64>> = Mutex::new(HashMap::new());
    static ref CHAT: Mutex<HashMap<String, CompModel>> = Mutex::new(HashMap::new());
    static ref MONITOR: Mutex<bool> = Mutex::new(true);
    static ref BOUNTY: Mutex<HashMap<String, BountySubmit>> = Mutex::new(HashMap::new());
}

#[tokio::main]
async fn main() {
    //run discord bot
    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;
    match Init::new().await {
        Ok(conf) => {
            let mut client = Client::builder(conf.discord.token.to_owned(), intents)
                .event_handler(Handler {
                    config: conf,
                    pedia: ItemPedia::default(),
                    image: Images::new().await.unwrap(),
                })
                .await
                .expect("Error creating client");
            if let Err(why) = client.start().await {
                println!("Client error: {:?}", why);
            }
        }
        Err(why) => {
            println!("serialize error: {}", why)
        }
    }
}
