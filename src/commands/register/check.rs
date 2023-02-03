use serenity::prelude::Context;
use serenity::all::*;
use crate::{Init,ErrorLog,PgConn,Components};



pub async fn run(ctx:&Context,cmd:&CommandInteraction,init:&Init){
    let mut err = ErrorLog::new(&ctx, init, &cmd.user).await;
    match PgConn::create(init, cmd.user.id.to_string()).await {
        Ok(mut pg) =>{
            match pg.get_char_id().await{
                Ok(data)=>{
                    let message;
                    if data.0 != 0{
                        message = format!("Your username is `{}` with user id `{}`",data.1,data.0);
                    }else {
                        message = "you dont have any account on this server".to_string();
                    }
                    if let Err(why) = cmd.create_response(&ctx.http, Components::interaction_response(&message, true)).await{
                        err.change_error(why.to_string(), "on check command", "just discord error,please consult".to_string());
                        err.log_error_channel().await;
                    }
                }
                Err(why)=>{
                    err.change_error(why.to_string(), "getting user information", "this is actually rare, please consult".to_string());
                    err.log_slash(cmd, false).await;
                }
            }
            pg.close().await;
        }
        Err(why) =>{
            err.change_error(why.to_string(), "getting postgres pool", "try again when connection to server more stable".to_string());
            err.log_slash(cmd, false).await;
        }
    }
}
