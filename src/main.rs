#![allow(dead_code)]

pub mod reusable;
pub mod commands;
pub mod material;
mod event;

use std::collections::HashMap;
use reusable::config::Init;
use reusable::component::error::ErrorLog;
use reusable::postgress::PgConn;
use reusable::component::{Components,MyErr,Mytrait,registered::Reg};
use event::{interaction::{ModalBundle,SlashBundle,ComponentBundle,Mybundle},message::MsgBundle};
use reusable::component::discord::AppReg;
use reusable::image_edit::Images;
use material::ItemPedia;
use serenity::prelude::{*, Mutex};
use event::Handler;
use lazy_static::lazy_static;

lazy_static!{
    static ref COOLDOWN:Mutex<HashMap<String,i64>> = Mutex::new(HashMap::new());
    static ref INIT:Mutex<Init> = Mutex::new(Init::block_new().unwrap());
}

#[tokio::main]
async fn main(){
    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    match Init::new().await{
        Ok(conf)=> {
            let mut client = Client::builder(conf.discord.token.to_owned(), intents)
                .event_handler(Handler{config:conf,pedia:ItemPedia::default(),image:Images::new().await.unwrap()})
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
