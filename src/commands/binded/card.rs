use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::{CommandDataOption, ApplicationCommandInteraction};
use serenity::prelude::Context;
use crate::{ErrorLog,Init,PgConn};

pub async fn run(_options: &[CommandDataOption],ctx:&Context,cmd:&ApplicationCommandInteraction,init:&Init){
    let mut error = ErrorLog::new(ctx,init,&cmd.user).await;
    match PgConn::create(init, &cmd.user.id.to_string()).await{
        Ok(pg)=>{
            match pg.get_card().await{
                Ok(cd)=>match cd{
                    Some(card)=>{
                        let path = card.get_path().0.to_owned();
                        if let Err(why)=cmd.create_interaction_response(&ctx.http, |m|{
                            card.card(m, &cmd.user, &path)
                        }).await{
                            error.change_error(why.to_string(), "card response", "connection problem");
                            error.log_error_channel().await;
                        }
                    }
                    None=>{
                        error.change_error("no message".to_string(), "card slash", "you dont have character binded in this server,try create or bind one");
                        error.log_slash(cmd, false).await;
                    }
                },
                Err(why)=>{
                    error.change_error(why.to_string(), "getting card", "database connection failure");
                    error.log_slash(cmd, false).await;
                }
            }
        }
        Err(why)=>{
            error.change_error(why.to_string(), "getting databse pool", "connection to database timed out, wit for server back to health");
            error.log_slash(cmd, false).await;
        }
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("card").description("show player curruent binded character's info")
}

