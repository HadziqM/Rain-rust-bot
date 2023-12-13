use serenity::all::*;
use crate::{MyErr,Mytrait,Mybundle,Components,PgConn,SlashBundle};


#[hertz::hertz_slash_normal(0,false)]
async fn slash(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    let mut username = "";
    let mut pass = "";
    for i in &bnd.cmd.data.options{
        if let CommandDataOptionValue::String(s) = &i.value{
            if i.name.as_str() == "username"{
                username = s.as_str();
            }else {
                pass = s.as_str();
            }
        }
    }
    let mut pg = PgConn::create(bnd.init, bnd.cmd.user.id.to_string()).await?;
    pg.change_password_with_username(pass, username).await?;
    Components::response(bnd, "Succeeded", true).await?;
    pg.close().await;
    Ok(())
}
