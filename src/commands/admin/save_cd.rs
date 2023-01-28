use serenity::{model::{prelude::interaction::{application_command::{ApplicationCommandInteraction, CommandDataOptionValue}, InteractionResponseType}, user::User}, prelude::Context};


use crate::{PgConn,ErrorLog,Init};

fn get(cmd:&ApplicationCommandInteraction)->Option<User>{
    for i in &cmd.data.options{
        if let Some(res)=&i.resolved{
            if let CommandDataOptionValue::User(x,_)=res{
                return Some(x.to_owned());
            }
        }
    }
    None
}
pub async fn run(ctx:&Context,cmd:&ApplicationCommandInteraction,init:&Init){
    let mut error = ErrorLog::new(ctx, init, &cmd.user).await;
    let user = match get(cmd){
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
            if let Err(why) = cmd.create_interaction_response(&ctx.http, |f|{
                f.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|m|{
                        m.content(&format!("{} save transfer cooldown succesfully reseted",user.to_string()))
                    })
            }).await{
                error.discord_error(why.to_string(), "reset cd").await;
            }
        }
        Err(why)=>{
            error.pgcon_error(why.to_string(), "reset cd", cmd).await;
        }
    };
    pg.close().await;
}
