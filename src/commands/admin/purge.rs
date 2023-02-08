use serenity::all::*;
use crate::{Init,Register,Components};


pub async fn run(ctx:&Context,cmd:&CommandInteraction,init:&Init){
    let mut user = User::default();
    for i in &cmd.data.options{
        if let CommandDataOptionValue::User(x)=&i.value{
            user = cmd.data.resolved.users.get(x).unwrap().to_owned();
        }
    }
    let mut reg = match Register::default_user(ctx, cmd, init, "purge command", &user).await{
        Some(x)=>x,
        None=>{return;}
    };
    match reg.pg.purge().await{
        Ok(_)=>{
            if let Err(why)=cmd.create_response(&ctx.http, Components::interaction_response("user already purged", true)).await{
                reg.error.discord_error(why.to_string(), "letting command finished").await;
            }
        }
        Err(why)=>{
            reg.error.pgcon_error(why.to_string(), "purging user", cmd).await;
        }
    }
    reg.pg.close().await
}
