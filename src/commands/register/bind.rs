use serenity::{prelude::Context, all::CommandInteraction};
use crate::{Init,Register};

pub async fn run(ctx:&Context,cmd:&CommandInteraction,init:&Init){
    Register::default(ctx, cmd, init, "switch command",true).await;
}
