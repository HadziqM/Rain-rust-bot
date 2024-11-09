use common::setting::SettingAll;
use lazy_static::lazy_static;
use serenity::all::*;
use std::{
    collections::HashMap,
    future::Future,
    sync::{Arc, RwLock, RwLockReadGuard},
};
use tokio::sync::RwLock;

use crate::error::MyError;

pub type MyResult<T> = Result<T, MyError>;

pub struct App {
    pub setting: Arc<RwLock<SettingAll>>,
}

pub struct DiscordHandler {
    pub app: Arc<App>,
    pub command_list: HashMap<String, InteractionHandler>,
}

pub struct InteractionHandler {
    pub handle: Fn(App, CommandInteraction, Context) -> Future<Output = MyResult<()>>,
    pub id: String,
    pub command: CreateCommand,
}

impl InteractionHandler {
    async fn handle_command(&self, app: App, cmd: CommandInteraction, ctx: Context) {
        if let Err(e) = self.handle(app, cmd, ctx).await {
            // TODO: Proper Error Handling
            log::error!("Getting error: {:#?}", e);
        }
    }
}

pub trait InteractionTrait {
    fn id() -> String;
    fn command() -> CreateCommand;
    async fn handle(app: App, cmd: CommandInteraction, ctx: Context) -> MyResult<()>;
    fn reg(&self) -> InteractionHandler {
        InteractionHandler {
            id: Self::id(),
            handle: Self::handle,
            command: Self::command(),
        }
    }
}

impl EventHandler for DiscordHandler {
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        todo!()
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Command(cmd) => {
                if let Some(hnd) = self.command_list.get(&cmd.data.name) {
                    hnd.handle_command(self.app.clone(), cmd, ctx).await;
                }
            }
            _ => {}
        }
        todo!()
    }
    async fn message(&self, ctx: Context, new_message: Message) {
        todo!()
    }
}
