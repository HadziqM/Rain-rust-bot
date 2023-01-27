use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use crate::{Init,Register};

pub async fn run(ctx:&Context,cmd:&ApplicationCommandInteraction,init:&Init){
    let did = cmd.user.id.to_string();
    let mut reg = match Register::default(ctx, cmd, init, &did, "card command").await{
        Some(r)=>r,
        None=>{return;}
    };
    match reg.pg.get_card(reg.cid).await{
        Ok(card)=>{
            let path = card.get_path().0.to_owned();
            if let Err(why)=cmd.create_interaction_response(&ctx.http, |m|{
                card.card(m, &cmd.user, &path)
            }).await{
                reg.error.change_error(why.to_string(), "card response", "connection problem");
                reg.error.log_error_channel().await;
                reg.pg.close().await;
            }
        }
        Err(why)=>{
            reg.error.change_error(why.to_string(), "getting card", "database connection failure");
            reg.error.log_slash(cmd, false).await;
            reg.pg.close().await;
        }
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("card").description("show player curruent binded character's info")
}

