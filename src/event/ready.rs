use std::num::NonZeroU64;

use serenity::gateway::ActivityData;
use serenity::model::prelude::{GuildId, UserId};
use serenity::model::prelude::Ready;
use serenity::prelude::*;
use crate::commands;
use crate::Init;


pub async fn ready(ctx:&Context, ready:Ready, init:&Init){
    let user = UserId(NonZeroU64::new(init.discord.author_id).unwrap()).to_user(&ctx.http).await.unwrap();
    println!("----------------------------------------------------------------");
    println!("-------------------------- START -------------------------------");
    println!("----------------------------------------------------------------");
    println!("🤖 Bot is running as {}",ready.user.tag());
    println!("🛠 {} is acknowledged as author",user.tag());
    let mut command = Vec::new();
    command.append(&mut commands::register::reg());
    command.append(&mut commands::binded::reg());
    command.append(&mut commands::admin::reg());
    for guild in &ready.guilds{
        let x = guild.id.to_partial_guild(&ctx.http).await.unwrap();
        println!("🏛 {} is on guild **{}**",&ready.user.tag(),&x.name);
        GuildId::set_application_commands(guild.id, &ctx.http,command.clone()).await.unwrap();
    }
    ctx.set_activity(Some(ActivityData::competing("I want to die")));
}
