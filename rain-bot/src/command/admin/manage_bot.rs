use appflow::Appflow;

use crate::all::*;

pub struct Restart;

#[async_trait]
impl CommandInteractionTrait for Restart {
    fn name(&self) -> &'static str {
        "restart"
    }

    fn command(&self) -> CreateCommand {
        AppReg::admin_slash(self.name(), "restart bot")
    }

    async fn handle_int(
        &self,
        app: Arc<App>,
        cmd: CommandInteraction,
        ctx: Context,
    ) -> MyResult<()> {
        Components::response(
            &cmd,
            &ctx,
            "Restarting the bot (check the log for detailed process)",
            true,
        )
        .await?;
        app.restart().await;
        Ok(())
    }
}
