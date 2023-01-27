use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use serenity::model::prelude::interaction::InteractionResponseType;
use crate::{Init,ErrorLog,PgConn};



pub async fn run(ctx:&Context,cmd:&ApplicationCommandInteraction,init:&Init){
    let mut err = ErrorLog::new(&ctx, init, &cmd.user).await;
    match PgConn::create(init, &cmd.user.id.to_string()).await {
        Ok(mut pg) =>{
            match pg.get_char_id().await{
                Ok(data)=>{
                    let message;
                    if data.0 != 0{
                        message = format!("Your username is `{}` with user id `{}`",data.1,data.0);
                    }else {
                        message = "you dont have any account on this server".to_string();
                    }
                    if let Err(why) = cmd.create_interaction_response(&ctx.http, |resp| {
                        resp.kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|msg|msg.content(message.as_str()).ephemeral(true))
                    }).await{
                        err.change_error(why.to_string(), "on check command", "just discord error,please consult");
                        err.log_error_channel().await;
                    }
                }
                Err(why)=>{
                    err.change_error(why.to_string(), "getting user information", "this is actually rare, please consult");
                    err.log_slash(cmd, false).await;
                }
            }
            pg.close().await;
        }
        Err(why) =>{
            err.change_error(why.to_string(), "getting postgres pool", "try again when connection to server more stable");
            err.log_slash(cmd, false).await;
        }
    }
}
