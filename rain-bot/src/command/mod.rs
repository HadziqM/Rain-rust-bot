use std::sync::Arc;

use serenity::{
    all::{CommandInteraction, Context, CreateCommand},
    async_trait,
};

use crate::setup::{App, CommandInteractionTrait, MyResult};

pub struct TestCommand;

#[async_trait]
impl CommandInteractionTrait for TestCommand {
    fn name(&self) -> String {
        "test".to_string()
    }
    fn command(&self) -> serenity::all::CreateCommand {
        CreateCommand::new("test")
    }
    async fn handle(&self, app: Arc<App>, cmd: CommandInteraction, ctx: Context) -> MyResult<()> {
        Ok(())
    }
}
