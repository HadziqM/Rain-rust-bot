use common::setting::SettingAll;
use serenity::all::*;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::error::MyError;

pub type MyResult<T> = Result<T, MyError>;

pub struct App {
    pub setting: Arc<RwLock<SettingAll>>,
}

pub struct DiscordHandler<T: CommandInteractionTrait> {
    pub app: Arc<App>,
    pub command_list: HashMap<String, T>,
}

#[async_trait]
pub trait CommandInteractionTrait: Sync + Send + 'static {
    async fn hand(&self, app: Arc<App>, cmd: CommandInteraction, ctx: Context) {
        if let Err(e) = Self::handle(app, cmd, ctx).await {
            // TODO: Proper error handling
            log::error!("there is error {:?}", e);
        }
    }
    async fn handle(app: Arc<App>, cmd: CommandInteraction, ctx: Context) -> MyResult<()>;
    fn command() -> CreateCommand;
    fn name() -> String;
}

#[async_trait]
impl<T: CommandInteractionTrait> EventHandler for DiscordHandler<T> {
    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        todo!()
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Command(cmd) => {
                if let Some(x) = self.command_list.get(&cmd.data.name) {
                    x.hand(self.app.clone(), cmd, ctx).await;
                }
            }
            Interaction::Component(_x) => {
                todo!()
            }
            _ => {}
        }
    }
    async fn message(&self, _ctx: Context, _new_message: Message) {
        todo!()
    }
}
