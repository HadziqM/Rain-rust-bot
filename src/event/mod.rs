pub mod interaction;
pub mod message;
mod ready;
mod paralel;
mod join;


use std::time::Duration;

use serenity::all::{Member, User, GuildId};
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
    async fn guild_member_addition(&self,ctx:Context,member:Member) {
        if self.config.bot_config.member_join{
            if let Err(why) = join::join(&ctx, &member, &self.config).await{
                eprintln!("member join fail : {why:?}")
            }
        }
    }
    async fn guild_member_removal(&self,ctx:Context,guild:GuildId,_:User,member:Option<Member>) {
        let mem = member.unwrap();
        if self.config.bot_config.member_leave{
            if let Err(why) = join::leave(&ctx,&mem,&guild,&self.config).await{
                eprintln!("member leave fail : {why:?}")
            }
        }
    }
}
