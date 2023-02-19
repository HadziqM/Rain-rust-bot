use serenity::all::*;
use crate::{MyErr,Components,Reg,SlashBundle,Mytrait,Mybundle};

#[hertz::hertz_slash_reg(60,false)]
async fn slash(bnd:&SlashBundle<'_>,mut reg:Reg<'_>)->Result<(),MyErr> {
    if let CommandDataOptionValue::String(pass) = &bnd.cmd.data.options.first().unwrap().value {
        reg.pg.change_user_password(pass.as_str(),reg.cid).await?;
        bnd.cmd.create_response(&bnd.ctx.http,Components::interaction_response("your password succesfully changed", true)).await?;
    }
    reg.pg.close().await;
    Ok(())
}
