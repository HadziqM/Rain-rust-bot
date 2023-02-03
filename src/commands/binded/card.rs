use serenity::{prelude::Context, all::CommandInteraction};
use crate::{Init,Register,PgConn,ErrorLog};

pub async fn run(ctx:&Context,cmd:&CommandInteraction,init:&Init){
    let mut reg = match Register::default(ctx, cmd, init, "card command",false).await{
        Some(r)=>r,
        None=>{return;}
    };
    match reg.pg.get_card(reg.cid).await{
        Ok(card)=>{
            if let Err(why)=cmd.create_response(&ctx.http,card.card(&cmd.user)).await{
                reg.error.change_error(why.to_string(), "card response", "connection problem".to_string());
                reg.error.log_error_channel().await;
                reg.pg.close().await;
            }
        }
        Err(why)=>{
            reg.error.change_error(why.to_string(), "getting card", "database connection failure".to_string());
            reg.error.log_slash(cmd, false).await;
            reg.pg.close().await;
        }
    }
}
pub async fn run_user(ctx:&Context,cmd:&CommandInteraction,init:&Init){
    let mut err = ErrorLog::new(ctx, init, &cmd.user).await;
    let user =match cmd.data.resolved.users.iter().next(){
        Some((_id,u))=>u,
        None=>{
            err.change_error("no user detected".to_string(), "card context", "idk what you do".to_string());
            err.log_slash(cmd, false).await;
            return;
        }
    };
    let mut pg = match PgConn::create(init, user.id.to_string()).await{
        Ok(p)=>p,
        Err(why)=>{
            err.pgcon_error(why.to_string(), "user context menu", cmd).await;
            return;
        }
    };
    match pg.get_user_data().await{
        Ok(dt)=>{
            if dt.cid == 0 {
                err.change_error(format!("{} doesnt have character selected",user.name),
                    "getting user card",format!("{} doesnt have account in this server or they doesnt select their character yet with `/switch`",user.to_string()));
                err.log_slash(cmd, false).await;
                return pg.close().await;
            }
            match pg.get_card(dt.cid).await{
                Ok(card)=>{
                    if let Err(why)=cmd.create_response(&ctx.http,card.card(&user)).await{
                        err.discord_error(why.to_string(), "show card").await;
                        return pg.close().await;
                    }
                }
                Err(why)=>{
                    err.pgcon_error(why.to_string(), "get card", cmd).await;
                    return pg.close().await;
                }
            }
        }
        Err(why)=>{
            err.pgcon_error(why.to_string(), "getting user data", cmd).await;
            return pg.close().await;
        }
    }
    pg.close().await;
}
