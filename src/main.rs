#![allow(dead_code)]

pub mod reusable;
pub mod commands;
mod event;

use reusable::config::*;
use serenity::{prelude::*, model::user::CurrentUser};
use event::Handler;


pub static mut USER:Option<CurrentUser> = None;
pub static mut CONFIG:Option<Init> = None;

#[tokio::main]
async fn main() {
    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    match get_config(){
        Ok(conf)=> {
            unsafe{
                CONFIG = Some(conf.clone());
            }
            let mut client = Client::builder(conf.discord.token, intents)
                .event_handler(Handler)
                .await
                .expect("Error creating client");
            if let Err(why) = client.start().await {
                println!("Client error: {:?}", why);
            }
        }
        Err(why)=>{
            println!("serialize error: {}",why)
        }
    }
}
