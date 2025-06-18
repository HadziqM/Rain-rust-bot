use crate::all::*;

pub struct GachaCommand;

#[async_trait]
impl CommandInteractionTrait for GachaCommand {
    fn name(&self) -> &'static str {
        "gacha"
    }
    fn command(&self) -> serenity::all::CreateCommand {
        AppReg::normal_slash(self.name(), "discord server gacha using gacha ticket")
            .add_option(AppReg::subcommand("single", "single gacha pull"))
            .add_option(AppReg::subcommand("multi", "multi gacha pull"))
    }
    async fn handle_int(
        &self,
        app: Arc<App>,
        cmd: CommandInteraction,
        ctx: Context,
    ) -> MyResult<()> {
        for data in &cmd.data.options {
            if let CommandDataOptionValue::SubCommand(_) = &data.value {
                let avatar_url = cmd.user().static_face();
                let did = cmd.discord_id();
                let name = &data.name;

                let pull = match name.as_str() {
                    "single" => 1,
                    _ => 10,
                };

                let byte = app.command_gacha(did, avatar_url, pull).await?;
                let resp = CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .add_file(CreateAttachment::bytes(byte, "gacha.png")),
                );

                cmd.create_response(&ctx.http, resp).await?;
            }
        }
        Ok(())
    }
}
