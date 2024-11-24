use crate::all::*;

struct ChangePassword;

#[async_trait]
impl CommandInteractionTrait for ChangePassword {
    fn name(&self) -> String {
        "change_password".to_string()
    }
    fn command(&self) -> serenity::all::CreateCommand {
        AppReg::normal_slash(self.name(), "change your in game account password")
    }
    async fn handle_int(
        &self,
        _app: Arc<App>,
        _cmd: CommandInteraction,
        _ctx: Context,
    ) -> MyResult<()> {
        Ok(())
    }
}

command_reg![ChangePassword];
