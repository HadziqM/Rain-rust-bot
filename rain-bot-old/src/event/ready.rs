use std::num::NonZeroU64;

use crate::commands;
use crate::reusable::component::shutdown::Shutdown;
use crate::Init;
use serenity::gateway::ActivityData;
use serenity::model::prelude::Ready;
use serenity::model::prelude::{GuildId, UserId};
use serenity::prelude::*;

pub async fn ready(ctx: &Context, ready: Ready, init: &Init) {
    let user = UserId(NonZeroU64::new(init.discord.author_id).unwrap())
        .to_user(&ctx.http)
        .await
        .unwrap();
    //load cache if exist
    if let Err(why) = Shutdown::load(ctx, init).await {
        println!("cant load cahed file: {why}")
    }
    //init save cache to file if exit background process
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        println!("\nTrying to Shutdown Gracefully");
        if let Err(why) = Shutdown::save().await {
            println!("cant save cache object: {why}")
        }
        std::process::exit(0);
    });
    println!("----------------------------------------------------------------");
    println!("-------------------------- START -------------------------------");
    println!("----------------------------------------------------------------");
    println!("🤖 Bot is running as {}", ready.user.tag());
    println!("🛠 {} is acknowledged as author", user.tag());
    let mut command = Vec::new();
    command.append(&mut commands::register::reg());
    command.append(&mut commands::binded::reg());
    command.append(&mut commands::misc::reg());
    command.append(&mut commands::guild::reg());
    command.append(&mut commands::admin::reg(init));
    if init.bot_config.gacha {
        command.append(&mut commands::gacha::reg());
    }
    if init.bot_config.server_market {
        command.append(&mut commands::market::reg());
    }
    if init.bot_config.bounty {
        command.append(&mut commands::bounty::reg());
    }
    for guild in &ready.guilds {
        let x = guild.id.to_partial_guild(&ctx.http).await.unwrap();
        println!("🏛 {} is on guild **{}**", &ready.user.tag(), &x.name);
        GuildId::set_application_commands(guild.id, &ctx.http, command.clone())
            .await
            .unwrap();
    }
    ctx.set_activity(Some(ActivityData::competing("I want to die")));
}
