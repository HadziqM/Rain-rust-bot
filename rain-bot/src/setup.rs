use common::setting::SettingAll;
use database::Db;
use serenity::all::*;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::error::{ErrorHandling, MyError};

pub type MyResult<T> = Result<T, MyError>;

pub struct App {
    pub setting: Arc<RwLock<SettingAll>>,
    pub db: Db,
}

pub struct DiscordHandler {
    pub app: Arc<App>,
    pub command_list: HashMap<String, Box<dyn CommandInteractionTrait>>,
    pub button_list: HashMap<String, Box<dyn ButtonInteractionTrait>>,
    pub modal_list: HashMap<String, Box<dyn ModalInteractionTrait>>,
}

impl DiscordHandler {
    pub fn new(app: Arc<App>) -> Self {
        Self {
            app,
            command_list: HashMap::new(),
            button_list: HashMap::new(),
            modal_list: HashMap::new(),
        }
    }
}

#[async_trait]
pub trait CommandInteractionTrait: Sync + Send + 'static {
    async fn hand(&self, app: Arc<App>, cmd: CommandInteraction, ctx: Context) {
        if let Err(e) = self.handle(app.clone(), cmd.clone(), ctx.clone()).await {
            e.log();
            let setting = app.setting.read().await;
            let err = ErrorHandling::new(e, &ctx, &setting, cmd.user.clone(), self.name()).await;
            if cmd
                .create_response(&ctx.http, err.response())
                .await
                .is_err()
            {
                err.channel_send(&ctx).await
            }
        }
    }
    async fn handle(&self, app: Arc<App>, cmd: CommandInteraction, ctx: Context) -> MyResult<()>;
    fn command(&self) -> CreateCommand;
    fn name(&self) -> String;
}
#[async_trait]
pub trait ButtonInteractionTrait: Sync + Send + 'static {
    async fn hand(&self, app: Arc<App>, cmd: ComponentInteraction, ctx: Context) {
        if let Err(e) = self.handle(app.clone(), cmd.clone(), ctx.clone()).await {
            e.log();
            let setting = app.setting.read().await;
            let err = ErrorHandling::new(e, &ctx, &setting, cmd.user.clone(), self.name()).await;
            if cmd
                .create_response(&ctx.http, err.response())
                .await
                .is_err()
            {
                err.channel_send(&ctx).await
            }
        }
    }
    async fn handle(&self, app: Arc<App>, cmd: ComponentInteraction, ctx: Context) -> MyResult<()>;
    fn name(&self) -> String;
}
#[async_trait]
pub trait ModalInteractionTrait: Sync + Send + 'static {
    async fn hand(&self, app: Arc<App>, cmd: ModalInteraction, ctx: Context) {
        if let Err(e) = self.handle(app.clone(), cmd.clone(), ctx.clone()).await {
            e.log();
            let setting = app.setting.read().await;
            let err = ErrorHandling::new(e, &ctx, &setting, cmd.user.clone(), self.name()).await;
            if cmd
                .create_response(&ctx.http, err.response())
                .await
                .is_err()
            {
                err.channel_send(&ctx).await
            }
        }
    }
    async fn handle(&self, app: Arc<App>, cmd: ModalInteraction, ctx: Context) -> MyResult<()>;
    fn name(&self) -> String;
}

#[async_trait]
impl EventHandler for DiscordHandler {
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
            Interaction::Component(cmd) => {
                if let Some(x) = self.button_list.get(&cmd.data.custom_id) {
                    x.hand(self.app.clone(), cmd, ctx).await;
                }
            }
            Interaction::Modal(cmd) => {
                if let Some(x) = self.modal_list.get(&cmd.data.custom_id) {
                    x.hand(self.app.clone(), cmd, ctx).await;
                }
            }
            _ => {}
        }
    }
    async fn message(&self, _ctx: Context, _new_message: Message) {
        todo!()
    }
}
