use serenity::all::*;
use crate::{MyErr,Components,Reg,SlashBundle,Mytrait,Mybundle};

#[hertz::hertz_slash_normal(60,false)]
async fn slash(bnd:&SlashBundle<'_>)->Result<(),MyErr> {
    let mut reg = Reg::only_check_alter(bnd, &bnd.cmd.user).await?;
    if let CommandDataOptionValue::String(pass) = &bnd.cmd.data.options.first().unwrap().value {
        reg.pg.change_user_password(pass.as_str(),reg.cid).await?;
        bnd.cmd.create_response(&bnd.ctx.http,Components::interaction_response("your password succesfully changed", true)).await?;
    }
    reg.pg.close().await;
    Ok(())
}
#[hertz::hertz_slash_normal(60,false)]
async fn psn(bnd:&SlashBundle<'_>)->Result<(),MyErr> {
    let mut reg = Reg::only_check_alter(bnd, &bnd.cmd.user).await?;
    if let CommandDataOptionValue::String(psn) = &bnd.cmd.data.options.first().unwrap().value {
        reg.pg.psn(psn, reg.cid).await?;
        bnd.cmd.create_response(&bnd.ctx.http,Components::interaction_response("your psn id is changed", true)).await?;
    }
    reg.pg.close().await;
    Ok(())
}
#[hertz::hertz_slash_normal(0,false)]
async fn check(bnd:&SlashBundle<'_>)->Result<(),MyErr> {
    if let Some(mut reg) = Reg::reverse_check(bnd, &bnd.cmd.user).await?{
        let user = reg.pg.get_user().await?;
        Components::response(bnd, &format!("Your Username is = {}",user.1), true).await?;
        reg.pg.close().await;
    };
    Ok(())
}
