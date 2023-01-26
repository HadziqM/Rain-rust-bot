use serenity::model::prelude::GuildId;
use serenity::model::prelude::{Ready, Activity};
use serenity::prelude::*;
use crate::commands;


pub async fn ready(ctx:Context, ready:Ready){
    let user = ready.user.name;
    println!("{} is running", &user);
    for guild in &ready.guilds{
        println!("{} is on guild **{}**",&user,guild.id);
        GuildId::set_application_commands(&guild.id, &ctx.http, |apps|{
            apps
                .create_application_command(|command| commands::id::register(command))
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::error::register(command))
                .create_application_command(|c|commands::register::interface::register(c))
                .create_application_command(|c|commands::register::create::register(c))
                .create_application_command(|c|commands::register::check::register(c))
                .create_application_command(|c|commands::register::change_pasword::register(c))
                .create_application_command(|c|commands::binded::card::register(c))
        }).await.unwrap();
    }
    ctx.set_activity(Activity::competing("i want to die")).await;
}

