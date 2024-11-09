use common::setting::SettingAll;
use serenity::all::*;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::error::{ErrorHandling, MyError};

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
        if let Err(e) = Self::handle(app.clone(), cmd.clone(), ctx.clone()).await {
            e.log();
            let setting = app.setting.read().await;
            let err = ErrorHandling::new(e, &ctx, &setting, cmd.user.clone(), Self::name()).await;
            if cmd
                .create_response(&ctx.http, err.response())
                .await
                .is_err()
            {
                err.channel_send(&ctx).await
            }
        }
    }
    async fn handle(app: Arc<App>, cmd: CommandInteraction, ctx: Context) -> MyResult<()>;
    fn command() -> CreateCommand;
    fn name() -> String;
}
#[async_trait]
pub trait ButtonInteractionTrait: Sync + Send + 'static {
    async fn hand(&self, app: Arc<App>, cmd: ComponentInteraction, ctx: Context) {
        if let Err(e) = Self::handle(app.clone(), cmd.clone(), ctx.clone()).await {
            e.log();
            let setting = app.setting.read().await;
            let err = ErrorHandling::new(e, &ctx, &setting, cmd.user.clone(), Self::name()).await;
            if cmd
                .create_response(&ctx.http, err.response())
                .await
                .is_err()
            {
                err.channel_send(&ctx).await
            }
        }
    }
    async fn handle(app: Arc<App>, cmd: ComponentInteraction, ctx: Context) -> MyResult<()>;
    fn name() -> String;
}
#[async_trait]
pub trait ModalInteractionTrait: Sync + Send + 'static {
    async fn hand(&self, app: Arc<App>, cmd: ModalInteraction, ctx: Context) {
        if let Err(e) = Self::handle(app.clone(), cmd.clone(), ctx.clone()).await {
            e.log();
            let setting = app.setting.read().await;
            let err = ErrorHandling::new(e, &ctx, &setting, cmd.user.clone(), Self::name()).await;
            if cmd
                .create_response(&ctx.http, err.response())
                .await
                .is_err()
            {
                err.channel_send(&ctx).await
            }
        }
    }
    async fn handle(app: Arc<App>, cmd: ModalInteraction, ctx: Context) -> MyResult<()>;
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
