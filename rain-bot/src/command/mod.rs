use std::sync::Arc;

use serenity::{
    all::{CommandInteraction, ComponentInteraction, Context, CreateCommand},
    async_trait,
};

use crate::setup::{App, ButtonInteractionTrait, CommandInteractionTrait, MyResult};

pub struct TestCommand;

#[async_trait]
impl CommandInteractionTrait for TestCommand {
    fn name(&self) -> String {
        "test".to_string()
    }
    fn command(&self) -> serenity::all::CreateCommand {
        CreateCommand::new("test")
    }
    async fn handle_int(
        &self,
        app: Arc<App>,
        cmd: CommandInteraction,
        ctx: Context,
    ) -> MyResult<()> {
        Ok(())
    }
}

#[async_trait]
impl ButtonInteractionTrait for TestCommand {
    fn name(&self) -> String {
        "test".to_string()
    }
    async fn handle_button(
        &self,
        app: Arc<App>,
        cmd: ComponentInteraction,
        ctx: Context,
    ) -> MyResult<()> {
        Ok(())
    }
}
