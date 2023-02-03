mod interaction;
mod ready;
mod paralel;
mod log;


use std::time::Duration;

use serenity::async_trait;
use serenity::builder::{CreateMessage, CreateEmbed};
use serenity::model::prelude::{Ready, Message, Interaction,Interaction::*};
use serenity::prelude::*;
use crate::reusable::config::*;
use interaction::*;


#[derive(Debug,Clone)]
pub struct Handler{
    pub config:Init
}

#[async_trait]
impl EventHandler for Handler{
    async fn interaction_create(&self, ctx: Context, inter:Interaction) {
        match inter {
            Modal(cmd)=>modal_command(&cmd.data.custom_id, &cmd, &ctx,&self.config).await,
            Command(cmd) => slash_command(&cmd.data.name, &cmd,&ctx, &self.config).await,
            Component(cmd) => button_command(&cmd.data.custom_id, &cmd,&ctx, &self.config).await,
            _=>println!("unhandled interaction")
        }
    }
    async fn ready(&self, ctx:Context, ready:Ready){
        ready::ready(&ctx, ready,&self.config).await;
        let init = self.config.clone();
        tokio::spawn(async move {
            loop {
                paralel::paralel_thread(&ctx, &init).await;
                tokio::time::sleep(Duration::from_secs(60*5)).await;
            }
        });
    }
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content==format!("{}test",self.config.discord.prefix) {
            if let Err(why) = msg.channel_id.send_message(&ctx.http,CreateMessage::new().embed(CreateEmbed::new().title("tested"))).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}
