use serenity::{prelude::Context, model::prelude::interaction::application_command::ApplicationCommandInteraction};

use super::{super::postgress, error:: error_rply};

#[derive(Debug)]
pub struct UserData {
    cid: i32,
    rid:i32
}

async fn user_check(did:&str)->Result<UserData,sqlx::Error>{
    let pool = postgress::basic::connection().await?;
    let cid = postgress::basic::get_user(did, &pool).await?;
    if cid == 0 {
        return Ok(UserData { cid: 0, rid: postgress::basic::registered(did, &pool).await? });
    }
    Ok(UserData { cid, rid: 0 })
}



pub async fn binding_check(ctx:&Context,cmd:&ApplicationCommandInteraction)->Option<i32>{
    let id = cmd.user.id.to_string();
    match user_check(&id).await {
        Ok(d) =>{
            if d.cid != 0 {
                return Some(d.cid);
            }else if d.rid != 0{
                //perform bind
                return None;
            }
            error_rply(ctx, "no error msg", "checking user data", "You Are Not Registered (create account) or Binded yet try to bind your account to discord or create account if dont have yet", cmd).await;
            None
        }
        Err(why) =>{
            error_rply(ctx, why.to_string().as_str(), "checking user user data", "connection to database timed out, just try again when traffic is more stable", cmd).await;
            None
        }
    }
}
