use serenity::prelude::Context;
use serenity::all::*;
use crate::{PgConn,ErrorLog,Init,Components};

pub async fn run(ctx:&Context, cmd:&CommandInteraction,init:&Init) {
    let option = cmd.data.options
        .get(0)
        .expect("idk").value;
    let mut error = ErrorLog::new(ctx, init,&cmd.user).await;
    if let CommandDataOptionValue::String(pass) = option {
        match PgConn::create(init,cmd.user.id.to_string()).await {
            Ok(pg) => {
                match pg.change_user_password(pass.as_str()).await{
                    Ok(p)=>{
                        let out = match p {
                            true => "your password succesfully changed",
                            false => "you dont have any account in this server",
                        };
                        if let Err(why) = cmd.create_response(&ctx.http,Components::interaction_response(out, true)).await{
                            error.change_error(why.to_string(),"change password", "discord connection problem,please consult".to_string());
                            error.log_error_channel().await;
                        }
                    }
                    Err(why)=>{
                        error.change_error(why.to_string(), "changing account", "this one rare, please consult".to_string());
                        error.log_slash(cmd, false).await;
                    }
                }
            }
            Err(why) => {
                error.change_error(why.to_string(),"change password", "wait for server to be more stable".to_string())
            }
        }
    }
}
