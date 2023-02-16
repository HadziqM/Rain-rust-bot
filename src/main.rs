#![allow(dead_code)]

pub mod reusable;
pub mod commands;
pub mod material;
mod event;

use reusable::config::Init;
use reusable::component::error::ErrorLog;
use reusable::component::registered::Register;
use reusable::postgress::PgConn;
use reusable::component::{Components,MyErr};
use reusable::component::discord::AppReg;
use material::ItemPedia;
use serenity::prelude::*;
use event::Handler;


#[tokio::main]
async fn main() {
    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    match Init::new().await{
        Ok(conf)=> {
            let mut client = Client::builder(conf.discord.token.to_owned(), intents)
                .event_handler(Handler{config:conf,pedia:ItemPedia::default()})
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
