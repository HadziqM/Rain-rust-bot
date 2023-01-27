use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use crate::{Init,Register};

pub async fn run(ctx:&Context,cmd:&ApplicationCommandInteraction,init:&Init){
    let did = cmd.user.id.to_string();
    Register::default(ctx, cmd, init, &did, "switch command",true).await;
}

