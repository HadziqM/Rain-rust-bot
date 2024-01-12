use crate::{Components, MyErr, Mybundle, Mytrait, Reg, SlashBundle};
use serenity::all::*;

#[hertz::hertz_slash_normal(0, false)]
async fn slash(bnd: &SlashBundle<'_>) -> Result<(), MyErr> {
    let mut user = User::default();
    for i in &bnd.cmd.data.options {
        if let CommandDataOptionValue::User(x) = &i.value {
            user = bnd.cmd.data.resolved.users.get(x).unwrap().to_owned();
        }
    }
    let mut reg = Reg::check_user(bnd, &user).await?;
    reg.pg.purge().await?;
    Components::response(bnd, "user already purged", true).await?;
    let member = bnd.cmd.member.clone().unwrap();
    Components::remove_role(*member, bnd, bnd.init.server_role.register_role).await?;
    reg.pg.close().await;
    Ok(())
}
