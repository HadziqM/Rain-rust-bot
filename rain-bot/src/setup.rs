use common::setting::SettingAll;
use database::Db;
use serenity::all::*;
use std::{collections::HashMap, ops::Deref, sync::Arc};
use tokio::sync::RwLock;

use crate::error::{ErrorHandling, MyError};

pub type MyResult<T> = Result<T, MyError>;

pub struct App {
    pub setting: Arc<RwLock<SettingAll>>,
    pub db: Db,
    pub pedia: Arc<material::ItemPedia>,
}

pub struct DiscordHandler {
    pub app: Arc<App>,
    pub command_list: HashMap<String, &'static dyn CommandInteractionTrait>,
    pub button_list: HashMap<String, &'static dyn ButtonInteractionTrait>,
    pub modal_list: HashMap<String, &'static dyn ModalInteractionTrait>,
}

#[async_trait]
pub trait CommandInteractionTrait: Sync + Send + 'static {
    async fn hand_int(&self, app: Arc<App>, cmd: CommandInteraction, ctx: Context) {
        if let Err(e) = self.handle_int(app.clone(), cmd.clone(), ctx.clone()).await {
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
    async fn handle_int(
        &self,
        app: Arc<App>,
        cmd: CommandInteraction,
        ctx: Context,
    ) -> MyResult<()>;
    fn command(&self) -> CreateCommand;
    fn name(&self) -> String;
}
#[async_trait]
pub trait ButtonInteractionTrait: Sync + Send + 'static {
    async fn hand_button(&self, app: Arc<App>, cmd: ComponentInteraction, ctx: Context) {
        if let Err(e) = self
            .handle_button(app.clone(), cmd.clone(), ctx.clone())
            .await
        {
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
    async fn handle_button(
        &self,
        app: Arc<App>,
        cmd: ComponentInteraction,
        ctx: Context,
    ) -> MyResult<()>;
    fn name(&self) -> String;
}
#[async_trait]
pub trait ModalInteractionTrait: Sync + Send + 'static {
    async fn hand_modal(&self, app: Arc<App>, cmd: ModalInteraction, ctx: Context) {
        if let Err(e) = self
            .handle_modal(app.clone(), cmd.clone(), ctx.clone())
            .await
        {
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
    async fn handle_modal(
        &self,
        app: Arc<App>,
        cmd: ModalInteraction,
        ctx: Context,
    ) -> MyResult<()>;
    fn name(&self) -> String;
}

inventory::collect!(Box<dyn CommandInteractionTrait>);
inventory::collect!(Box<dyn ButtonInteractionTrait>);
inventory::collect!(Box<dyn ModalInteractionTrait>);

impl DiscordHandler {
    pub fn new(app: Arc<App>) -> Self {
        let command_list = inventory::iter::<Box<dyn CommandInteractionTrait>>()
            .map(|x| (x.name(), x.deref()))
            .collect();

        let button_list = inventory::iter::<Box<dyn ButtonInteractionTrait>>()
            .map(|x| (x.name(), x.deref()))
            .collect();

        let modal_list = inventory::iter::<Box<dyn ModalInteractionTrait>>()
            .map(|x| (x.name(), x.deref()))
            .collect();

        Self {
            app,
            command_list,
            button_list,
            modal_list,
        }
    }
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
                    x.hand_int(self.app.clone(), cmd, ctx).await;
                }
            }
            Interaction::Component(cmd) => {
                if let Some(x) = self.button_list.get(&cmd.data.custom_id) {
                    x.hand_button(self.app.clone(), cmd, ctx).await;
                }
            }
            Interaction::Modal(cmd) => {
                if let Some(x) = self.modal_list.get(&cmd.data.custom_id) {
                    x.hand_modal(self.app.clone(), cmd, ctx).await;
                }
            }
            _ => {}
        }
    }
    async fn message(&self, _ctx: Context, _new_message: Message) {
        todo!()
    }
}
