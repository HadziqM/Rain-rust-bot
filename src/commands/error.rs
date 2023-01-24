use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::{CommandDataOption, ApplicationCommandInteraction};
use serenity::prelude::Context;
use serenity::model::prelude::interaction::InteractionResponseType;

use crate::reusable::component::error::error;
use crate::reusable::config::Init;

pub async fn run(_options: &[CommandDataOption],ctx:&Context,cmd:&ApplicationCommandInteraction,init:&Init){
        if let Err(why) = cmd.create_interaction_response(&ctx.http, |resp| {
        resp.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|msg|msg.content(""))
    }).await{
        error(ctx, why.to_string().as_str(), "error command", "its just test",init,&cmd.user).await;
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("error").description("error")
}

