mod interaction;
mod ready;
mod paralel;
mod message;


use std::time::Duration;

use serenity::async_trait;
use serenity::model::prelude::{Ready, Message, Interaction,Interaction::*};
use serenity::prelude::*;
use crate::reusable::config::*;
use interaction::*;
use crate::material::ItemPedia;


#[derive(Debug,Clone)]
pub struct Handler{
    pub config:Init,
    pub pedia:ItemPedia
}

#[async_trait]
impl EventHandler for Handler{
    async fn interaction_create(&self, ctx: Context, inter:Interaction) {
        match inter {
            Modal(cmd)=>modal_command(&cmd.data.custom_id, &cmd, &ctx,&self.config).await,
            Command(cmd) => slash_command(&cmd.data.name, &cmd,&ctx, &self.config,&self.pedia).await,
            Component(cmd) => button_command(&cmd.data.custom_id, &cmd,&ctx, &self.config).await,
            Autocomplete(cmd)=>autocomplete_command(&cmd.data.name, &cmd, &ctx, &self.config,&self.pedia).await,
            _=>println!("unhandled interaction")
        }
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
                state = paralel::paralel_thread(&ctx, &init,state,log).await;
                log_count += 1;
                tokio::time::sleep(Duration::from_secs(60)).await;
            }
        });
    }
    async fn message(&self, ctx: Context, msg: Message) {
        message::msg_handler(ctx,msg,self.config.clone(),self.pedia.clone()).await
    }
}
