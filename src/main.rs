#![allow(dead_code)]

pub mod reusable;
pub mod commands;
pub mod material;
mod event;

use reusable::config::{Init,get_config};
use reusable::component::error::ErrorLog;
use reusable::component::registered::Register;
use reusable::postgress::PgConn;
use reusable::component::Components;
use reusable::component::discord::AppReg;
use serenity::prelude::*;
use event::Handler;


#[tokio::main]
async fn main() {
    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    match get_config(){
        Ok(conf)=> {
            let mut client = Client::builder(conf.discord.token.to_owned(), intents)
                .event_handler(Handler{config:conf})
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
