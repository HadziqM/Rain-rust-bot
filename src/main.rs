#![allow(dead_code)]

pub mod reusable;
pub mod commands;
mod event;

use serenity::prelude::*;
use event::Handler;

#[tokio::main]
async fn main() {
    if let Ok(conf) = reusable::config::get_config(){
        let mut client = Client::builder(conf.token, GatewayIntents::empty())
            .event_handler(Handler)
            .await
            .expect("Error creating client");
        if let Err(why) = client.start().await {
            println!("Client error: {:?}", why);
        }
    }
}
