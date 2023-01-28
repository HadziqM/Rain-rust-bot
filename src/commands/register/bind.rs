use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use crate::{Init,Register};

pub async fn run(ctx:&Context,cmd:&ApplicationCommandInteraction,init:&Init){
    Register::default(ctx, cmd, init, "switch command",true).await;
}

