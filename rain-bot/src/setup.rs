use common::setting::SettingAll;
use database::Db;
use log::{error, info};
use serenity::all::*;
pub use std::{
    collections::HashMap,
    ops::Deref,
    sync::{Arc, LazyLock},
};
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
    pub command_list: HashMap<String, Box<dyn CommandInteractionTrait>>,
    pub button_list: HashMap<String, Box<dyn ButtonInteractionTrait>>,
    pub modal_list: HashMap<String, Box<dyn ModalInteractionTrait>>,
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
            let err =
                ErrorHandling::new(e, &ctx, &setting, cmd.user.clone(), self.name_btn()).await;
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
    fn name_btn(&self) -> String;
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
            let err =
                ErrorHandling::new(e, &ctx, &setting, cmd.user.clone(), self.name_mdl()).await;
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
    fn name_mdl(&self) -> String;
}

impl DiscordHandler {
    pub fn new(app: Arc<App>) -> Self {
        Self {
            app,
            command_list: crate::command::reg_command(),
            button_list: crate::command::reg_button(),
            modal_list: crate::command::reg_modal(),
        }
    }
}

#[async_trait]
impl EventHandler for DiscordHandler {
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        info!("🤖 Bot is running as {}", data_about_bot.user.tag());

        let command = self
            .command_list
            .values()
            .map(|x| x.command())
            .collect::<Vec<_>>();

        for guild in &data_about_bot.guilds {
            match guild.id.set_commands(&ctx.http, command.clone()).await {
                Ok(_) => {
                    info!(
                        "🎉 Successfully registered commands for {} guild",
                        guild.id.to_string()
                    );
                }
                Err(err) => {
                    error!(
                        "🚫 Failed to register commands for {} guild: {}",
                        guild.id.to_string(),
                        err
                    );
                }
            }
        }

        ctx.set_activity(Some(ActivityData::playing("🎮 Rain Erupe")));
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
