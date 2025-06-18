use std::str::FromStr;

use common::setting::SettingList;
use strum::IntoEnumIterator;

use crate::all::*;

pub struct SetConfig;
pub struct ReadConfig;

#[async_trait]
impl CommandInteractionTrait for ReadConfig {
    fn name(&self) -> &'static str {
        "config_read"
    }
    fn command(&self) -> CreateCommand {
        let mut x = AppReg::admin_slash(self.name(), "send configuration");
        for i in SettingList::iter() {
            x = x.add_option(AppReg::subcommand(
                i.to_string(),
                format!("send {i} configuration"),
            ))
        }
        x
    }

    async fn handle_int(
        &self,
        _app: Arc<App>,
        cmd: CommandInteraction,
        ctx: Context,
    ) -> MyResult<()> {
        for data in &cmd.data.options {
            if let CommandDataOptionValue::SubCommand(_) = &data.value {
                let name = &data.name;
                let setting = SettingList::from_str(name).unwrap();

                let resp = CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .add_file(CreateAttachment::path(setting.path()).await?),
                );

                cmd.create_response(&ctx.http, resp).await?;
            }
        }

        Ok(())
    }
}
#[async_trait]
impl CommandInteractionTrait for SetConfig {
    fn name(&self) -> &'static str {
        "config_set"
    }
    fn command(&self) -> CreateCommand {
        let mut x = AppReg::admin_slash(self.name(), "change configuration");
        for i in SettingList::iter() {
            x = x.add_option(
                AppReg::subcommand(i.to_string(), format!("change {i} configuration"))
                    .add_sub_option(AppReg::att_option("attachment", "json configuration")),
            )
        }
        x
    }

    async fn handle_int(
        &self,
        app: Arc<App>,
        cmd: CommandInteraction,
        ctx: Context,
    ) -> MyResult<()> {
        for data in &cmd.data.options {
            if let CommandDataOptionValue::SubCommand(_) = &data.value {
                let name = &data.name;
                if let Some(resolved) = cmd.data.resolved.attachments.values().next() {
                    let byte = resolved.download().await?;
                    let setting = SettingList::from_str(name).unwrap();

                    let mut all = app.setting.write().await;
                    setting
                        .validete_and_change(&byte, &mut all)
                        .map_err(|x| MyError::from(x.as_str()))?;

                    tokio::fs::write(setting.path(), byte).await?;

                    Components::response(&cmd, &ctx, "Successfully changed the setting", true)
                        .await?
                }
            }
        }
        Ok(())
    }
}
