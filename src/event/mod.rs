pub mod interaction;
pub mod ready;
pub mod paralel;


use std::time::Duration;

use serenity::async_trait;
use serenity::model::prelude::{Ready, Message};
use serenity::prelude::*;
use serenity::model::application::interaction::{Interaction::*,Interaction};
use crate::reusable::config::*;
use interaction::*;


#[derive(Debug,Clone)]
pub struct Handler{
    pub config:Init
}

#[async_trait]
impl EventHandler for Handler{
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            ModalSubmit(cmd)=>modal_command(&cmd.data.custom_id, &cmd, &ctx,&self.config).await,
            ApplicationCommand(cmd) => slash_command(&cmd.data.name, &cmd,&ctx, &self.config).await,
            MessageComponent(cmd) => button_command(&cmd.data.custom_id, &cmd,&ctx, &self.config).await,
            _=>println!("unhandled interaction")
        }
    }
    async fn ready(&self, ctx:Context, ready:Ready){
        ready::ready(&ctx, ready,&self.config).await;
        let init = self.config.clone();
        tokio::spawn(async move {
            loop {
                paralel::paralel_thread(&ctx, &init).await;
                tokio::time::sleep(Duration::from_secs(60)).await;
            }
        });
    }
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content==format!("{}test",self.config.discord.prefix) {
            if let Err(why) = msg.channel_id.send_message(&ctx.http, |m|{
                m.add_embed(|e|e.title("tested"))
            }).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}
