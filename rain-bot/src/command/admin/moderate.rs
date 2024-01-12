use crate::{error::MyErr, setup::{Context, AppData}};


/// to purge user register data
#[poise::command(slash_command)]
async fn purge(
    ctx: Context<'_>,
    #[description = "The user that need to be purged"]
    user: serenity::all::User,
) -> Result<(),MyErr> {
    ctx.data().db.purge(&user.id.to_string()).await?;
    ctx.say("user succesfully purged").await?;
    Ok(())
}

pub fn reg() -> Vec<poise::Command<AppData,MyErr>> {
    vec![purge()]
}
