use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::{CommandDataOption, ApplicationCommandInteraction};
use serenity::prelude::Context;
use serenity::model::prelude::interaction::InteractionResponseType;

pub async fn run(_options: &[CommandDataOption],ctx:&Context,cmd:&ApplicationCommandInteraction){
    let out = "Hey, I'm alive!".to_string();
        if let Err(why) = cmd.create_interaction_response(&ctx.http, |resp| {
        resp.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|msg|msg.content(out.as_str()))
    }).await{
        println!("cannot respond to slash command: {}",why)
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}

