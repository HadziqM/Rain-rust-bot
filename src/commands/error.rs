use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use serenity::model::prelude::interaction::InteractionResponseType;
use crate::{Init,ErrorLog};

pub async fn run(ctx:&Context,cmd:&ApplicationCommandInteraction,init:&Init){
        if let Err(why) = cmd.create_interaction_response(&ctx.http, |resp| {
        resp.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|msg|msg.content(""))
    }).await{
        let mut err = ErrorLog::new(&ctx, init, &cmd.user).await;
        err.change_error(why.to_string(), "error command", "it just test woles");
        err.log_error_channel().await;
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("error").description("error")
}

