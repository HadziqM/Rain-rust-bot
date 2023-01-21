use serenity::{model::prelude::interaction::application_command::ApplicationCommandInteraction, prelude::Context};
use crate::commands;


pub async fn slash_command(command:&str,cmd:&ApplicationCommandInteraction,ctx:&Context){
    match command{
        "ping" => commands::ping::run(&cmd.data.options,ctx,cmd).await,
        "id" =>commands::id::run(&cmd.data.options,ctx,cmd).await,
        _=> println!("{} isnt slash command",command)
    }
}

