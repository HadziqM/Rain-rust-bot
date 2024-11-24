use crate::all::*;

struct Test;
struct TestError;

#[async_trait]
impl CommandInteractionTrait for Test {
    fn name(&self) -> String {
        "test".to_string()
    }

    fn command(&self) -> serenity::all::CreateCommand {
        AppReg::normal_slash(self.name(), "test")
    }

    async fn handle_int(
        &self,
        _app: Arc<App>,
        _cmd: CommandInteraction,
        _ctx: Context,
    ) -> MyResult<()> {
        Components::response(&_cmd, &_ctx, "tested", true).await?;
        Ok(())
    }
}

#[async_trait]
impl CommandInteractionTrait for TestError {
    fn name(&self) -> String {
        "test_error".to_string()
    }
    fn command(&self) -> serenity::all::CreateCommand {
        AppReg::normal_slash(self.name(), "test")
    }
    async fn handle_int(
        &self,
        _app: Arc<App>,
        _cmd: CommandInteraction,
        _ctx: Context,
    ) -> MyResult<()> {
        Err("error tested".into())
    }
}

command_reg![Test, TestError];
