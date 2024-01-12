pub mod interaction;
mod join;
pub mod message;
mod paralel;
mod ready;

use std::time::Duration;

use crate::material::ItemPedia;
use crate::reusable::config::*;
use crate::Images;
use serenity::all::{GuildId, Member, User};
use serenity::async_trait;
use serenity::model::prelude::{Interaction, Message, Ready};
use serenity::prelude::*;

use self::interaction::handled;

#[derive(Debug, Clone)]
pub struct Handler {
    pub config: Init,
    pub pedia: ItemPedia,
    pub image: Images,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, inter: Interaction) {
        handled(&ctx, &inter, &self.pedia, &self.config, &self.image).await;
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        ready::ready(&ctx, ready, &self.config).await;
        let init = self.config.clone();
        tokio::spawn(async move {
            let mut state = 0;
            let mut log_count: u64 = 0;
            loop {
                let mut log = false;
                if log_count % 5 == 0 {
                    log = true
                }
                state = paralel::handle(&ctx, &init, state, log).await;
                log_count += 1;
                tokio::time::sleep(Duration::from_secs(60)).await;
            }
        });
    }
    async fn message(&self, ctx: Context, msg: Message) {
        message::msg_handler(&ctx, &msg, &self.config, &self.pedia).await
    }
    async fn guild_member_addition(&self, ctx: Context, member: Member) {
        if self.config.bot_config.member_join {
            if let Err(why) = join::join(&ctx, &member, &self.config).await {
                eprintln!("member join fail : {why:?}")
            }
        }
    }
    async fn guild_member_removal(
        &self,
        ctx: Context,
        guild: GuildId,
        user: User,
        _member: Option<Member>,
    ) {
        if self.config.bot_config.member_leave {
            if let Err(why) = join::leave(&ctx, &user, &guild, &self.config).await {
                eprintln!("member leave fail : {why:?}")
            }
        }
    }
}
