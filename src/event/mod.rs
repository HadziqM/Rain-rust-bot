pub mod interaction;
mod ready;
mod paralel;
pub mod message;


use std::time::Duration;

use serenity::async_trait;
use serenity::model::prelude::{Ready, Message, Interaction};
use serenity::prelude::*;
use crate::reusable::config::*;
use crate::material::ItemPedia;
use crate::Images;

use self::interaction::handled;


#[derive(Debug,Clone)]
pub struct Handler{
    pub config:Init,
    pub pedia:ItemPedia,
    pub image:Images,
}

#[async_trait]
impl EventHandler for Handler{
    async fn interaction_create(&self, ctx: Context, inter:Interaction) {
        handled(&ctx, &inter, &self.pedia, &self.config, &self.image).await;
    }
    async fn ready(&self, ctx:Context, ready:Ready){
        ready::ready(&ctx, ready,&self.config).await;
        let init = self.config.clone();
        tokio::spawn(async move {
            let mut state = 0;
            let mut log_count:u64 = 0;
            loop {
                let mut log = false;
                if log_count%5 == 0{
                    log = true
                }
                state = paralel::handle(&ctx, &init,state,log).await;
                log_count += 1;
                tokio::time::sleep(Duration::from_secs(60)).await;
            }
        });
    }
    async fn message(&self, ctx: Context, msg: Message) {
        message::msg_handler(&ctx,&msg,&self.config,&self.pedia).await
    }
}
