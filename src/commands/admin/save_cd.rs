use serenity::{all::{CommandInteraction, User, CommandDataOptionValue}, prelude::Context, builder::{CreateInteractionResponse, CreateInteractionResponseMessage}};

use crate::{PgConn,ErrorLog,Init};

async fn get(cmd:&CommandInteraction,ctx:&Context)->Option<User>{
    for i in &cmd.data.options{
        if let CommandDataOptionValue::User(x) = i.value{
            let r = match x.to_user(&ctx.http).await{
                Ok(y)=>Some(y),
                Err(_)=>{continue;}
            };
            return r;
        }
    }
    None
}
pub async fn run(ctx:&Context,cmd:&CommandInteraction,init:&Init){
    let mut error = ErrorLog::new(ctx, init, &cmd.user).await;
    let user = match get(cmd,ctx).await{
        Some(x)=>x,
        None=>{
            error.change_error("idk what you do".to_owned(), "reset cd", "please redo".to_string());
            error.log_slash(cmd, false).await;
            return ;
        }
    };
    let mut pg = match PgConn::create(init,user.id.to_string()).await {
        Ok(x) => x,
        Err(why) => {
            error.pgcon_error(why.to_string(), "reset cd", cmd).await;
            return ;
        },
    };
    match pg.reset_cd().await{
        Ok(_)=>{
            if let Err(why) = cmd.create_response(&ctx.http,CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
                    .content(format!("{} save cooldown already reseted",user.to_string())))).await{
                error.discord_error(why.to_string(), "reset cd").await;
            }
        }
        Err(why)=>{
            error.pgcon_error(why.to_string(), "reset cd", cmd).await;
        }
    };
    pg.close().await;
}
