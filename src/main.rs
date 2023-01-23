#![allow(dead_code)]

pub mod reusable;
pub mod commands;
mod event;

use reusable::config::{Init,get_config};
use serenity::prelude::*;
use event::Handler;

pub static mut CONFIG:Init = Init::default();

#[tokio::main]
async fn main() {
    if let Ok(conf) = get_config(){
        unsafe{
            CONFIG = conf.clone();
        }
        let mut client = Client::builder(conf.discord.token, GatewayIntents::empty())
            .event_handler(Handler)
            .await
            .expect("Error creating client");
        if let Err(why) = client.start().await {
            println!("Client error: {:?}", why);
        }
    }
}
