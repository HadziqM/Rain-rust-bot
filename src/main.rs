#![allow(dead_code)]

pub mod reusable;
pub mod commands;
mod event;

use reusable::config::{Init,get_config};
use reusable::component::error::ErrorLog;
use reusable::component::registered::Register;
use reusable::postgress::PgConn;
use reusable::component::Components;
use serenity::prelude::*;
use event::Handler;



#[tokio::main]
async fn main() {
    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    match get_config(){
        Ok(conf)=> {
            let config = conf.clone();
            let mut client = Client::builder(conf.discord.token, intents)
                .event_handler(Handler{config})
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
