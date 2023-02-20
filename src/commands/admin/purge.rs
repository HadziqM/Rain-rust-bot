use serenity::all::*;
use crate::{Reg,SlashBundle,Components,MyErr,Mytrait,Mybundle};

#[hertz::hertz_slash_normal(0,false)]
async fn slash(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    let mut user = User::default();
    for i in &bnd.cmd.data.options{
        if let CommandDataOptionValue::User(x)=&i.value{
            user = bnd.cmd.data.resolved.users.get(x).unwrap().to_owned();
        }
    }
    let mut reg = Reg::check_user(bnd, &user).await?;
    reg.pg.purge().await?;
    Components::response(bnd, "user already purged", true).await?;
    reg.pg.close().await;
    Ok(())
}
