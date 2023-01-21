#![allow(dead_code)]

pub mod reusable;
pub mod commands;
mod event;

use reusable::config::{Init,get_config};
use serenity::prelude::*;
use event::Handler;

pub static mut CONFIG:Init = Init {
    token:String::new(),
    prefix:String::new(),
    err_channel:String::new()
};

#[tokio::main]
async fn main() {
    if let Ok(conf) = get_config(){
        unsafe{
            CONFIG = conf.clone();
        }
        let mut client = Client::builder(conf.token, GatewayIntents::empty())
            .event_handler(Handler)
            .await
            .expect("Error creating client");
        if let Err(why) = client.start().await {
            println!("Client error: {:?}", why);
        }
    }
}
