use crate::all::*;
pub struct AddPsn;

pub async fn change_password(app: Arc<App>, cmd: CommandInteraction, ctx: Context) -> MyResult<()> {
    let data = app.only_register_user(&cmd.user).await?;
    if let CommandDataOptionValue::String(password) = &cmd.data.options.first().unwrap().value {
        app.db.change_password(password, data.uid).await?;
        Components::response(&cmd, &ctx, "Password changed", true).await?;
    }
    Ok(())
}
pub async fn add_psn(app: Arc<App>, cmd: CommandInteraction, ctx: Context) -> MyResult<()> {
    let data = app.only_register_user(&cmd.user).await?;
    if let CommandDataOptionValue::String(psn_id) = &cmd.data.options.first().unwrap().value {
        app.db
            .change_psn(Some(psn_id.to_string()), data.uid)
            .await?;
        Components::response(&cmd, &ctx, "PSN ID Added/changed", true).await?;
    }
    Ok(())
}
pub async fn unlink_psn(app: Arc<App>, cmd: CommandInteraction, ctx: Context) -> MyResult<()> {
    let data = app.only_register_user(&cmd.user).await?;
    app.db.change_psn(None, data.uid).await?;
    Components::response(&cmd, &ctx, "PSN ID unlinked", true).await?;
    Ok(())
}
