use serenity::model::prelude::{GuildId, UserId};
use serenity::model::prelude::{Ready, Activity};
use serenity::prelude::*;
use crate::commands;
use crate::reusable::config::Init;


pub async fn ready(ctx:Context, ready:Ready, init:&Init){
    let user = UserId(init.discord.author_id).to_user(&ctx.http).await.unwrap();
    println!("----------------------------------------------------------------");
    println!("-------------------------- START -------------------------------");
    println!("----------------------------------------------------------------");
    println!("🤖 Bot is running as {}",ready.user.tag());
    println!("🛠 {} is acknowledged as author",user.tag());
    for guild in &ready.guilds{
        let x = guild.id.to_partial_guild(&ctx.http).await.unwrap();
        println!("🏛 {} is on guild **{}**",&ready.user.tag(),&x.name);
        GuildId::set_application_commands(&guild.id, &ctx.http, |apps|{
            apps
                .create_application_command(|command| commands::id::register(command))
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::error::register(command))
                .create_application_command(|c|commands::register::interface::register(c))
                .create_application_command(|c|commands::register::create::register(c))
                .create_application_command(|c|commands::register::check::register(c))
                .create_application_command(|c|commands::register::bind::register(c))
                .create_application_command(|c|commands::register::change_pasword::register(c))
                .create_application_command(|c|commands::binded::card::register(c))
        }).await.unwrap();
    }
    ctx.set_activity(Activity::competing("i want to die")).await;
}

