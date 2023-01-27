use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use crate::{Init,Register};


pub async fn run(ctx:&Context,cmd:&ApplicationCommandInteraction,init:&Init){
    let did = cmd.user.id.to_string();
    Register::default(ctx, cmd, init, &did, "switch command",true).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("switch").description("switch your own binded character for server event")
}

