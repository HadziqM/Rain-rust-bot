use crate::MyApp;
use log::{debug, error, info};
use serenity::all::*;
pub use std::{collections::HashMap, ops::Deref, sync::Arc};

use crate::error::{CommandLocationType, ErrorHandling, MyError};

pub type MyResult<T> = Result<T, MyError>;

pub struct DiscordHandler {
    pub app: Arc<MyApp>,
    pub command_list: HashMap<String, Box<dyn CommandInteractionTrait>>,
    pub button_list: HashMap<String, Box<dyn ButtonInteractionTrait>>,
    pub modal_list: HashMap<String, Box<dyn ModalInteractionTrait>>,
    pub message_list: HashMap<String, Box<dyn MessageCommandTrait>>,
}

#[async_trait]
pub trait CommandInteractionTrait: Sync + Send + 'static {
    async fn hand_int(&self, app: Arc<MyApp>, cmd: CommandInteraction, ctx: Context) {
        if let Err(e) = self.handle_int(app.clone(), cmd.clone(), ctx.clone()).await {
            let setting = app.setting.read().await;
            let err = ErrorHandling::new(
                e,
                &ctx,
                &setting,
                cmd.user.clone(),
                self.name(),
                CommandLocationType::Slash,
            )
            .await;
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
        app: Arc<MyApp>,
        cmd: CommandInteraction,
        ctx: Context,
    ) -> MyResult<()>;
    fn command(&self) -> CreateCommand;
    fn name(&self) -> String;
}
#[async_trait]
pub trait ButtonInteractionTrait: Sync + Send + 'static {
    async fn hand_button(&self, app: Arc<MyApp>, cmd: ComponentInteraction, ctx: Context) {
        if let Err(e) = self
            .handle_button(app.clone(), cmd.clone(), ctx.clone())
            .await
        {
            let setting = app.setting.read().await;
            let err = ErrorHandling::new(
                e,
                &ctx,
                &setting,
                cmd.user.clone(),
                self.name_btn(),
                CommandLocationType::Button,
            )
            .await;
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
        app: Arc<MyApp>,
        cmd: ComponentInteraction,
        ctx: Context,
    ) -> MyResult<()>;
    fn name_btn(&self) -> String;
}
#[async_trait]
pub trait ModalInteractionTrait: Sync + Send + 'static {
    async fn hand_modal(&self, app: Arc<MyApp>, cmd: ModalInteraction, ctx: Context) {
        if let Err(e) = self
            .handle_modal(app.clone(), cmd.clone(), ctx.clone())
            .await
        {
            let setting = app.setting.read().await;
            let err = ErrorHandling::new(
                e,
                &ctx,
                &setting,
                cmd.user.clone(),
                self.name_mdl(),
                CommandLocationType::Modal,
            )
            .await;
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
        app: Arc<MyApp>,
        cmd: ModalInteraction,
        ctx: Context,
    ) -> MyResult<()>;
    fn name_mdl(&self) -> String;
}
#[async_trait]
pub trait MessageCommandTrait: Sync + Send + 'static {
    async fn hand_msg(&self, app: Arc<MyApp>, cmd: Message, ctx: Context) {
        if let Err(e) = self.handle_msg(app.clone(), cmd.clone(), ctx.clone()).await {
            let setting = app.setting.read().await;
            let err = ErrorHandling::new(
                e,
                &ctx,
                &setting,
                cmd.author.clone(),
                self.name_msg(),
                CommandLocationType::Modal,
            )
            .await;
            if cmd
                .channel_id
                .send_message(&ctx.http, CreateMessage::new().embed(err.embed()))
                .await
                .is_err()
            {
                err.channel_send(&ctx).await
            }
        }
    }
    async fn handle_msg(&self, app: Arc<MyApp>, cmd: Message, ctx: Context) -> MyResult<()>;
    fn name_msg(&self) -> String;
}

impl DiscordHandler {
    pub fn new(app: Arc<MyApp>) -> Self {
        Self {
            app,
            command_list: crate::command::reg_command(),
            button_list: crate::command::reg_button(),
            modal_list: crate::command::reg_modal(),
            message_list: crate::command::reg_message(),
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
            let name = guild.id.to_partial_guild(&ctx.http).await.unwrap().name;
            match guild.id.set_commands(&ctx.http, command.clone()).await {
                Ok(_) => {
                    info!("🎉 Successfully registered commands for {name} guild",);
                }
                Err(err) => {
                    error!("🚫 Failed to register commands for guild: {name} error: {err}",);
                }
            }
        }

        debug!(" Delete all global commands");
        Command::set_global_commands(&ctx.http, vec![]).await.ok();

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

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("?") && !msg.author.bot {
            let name = msg.content.split_whitespace().next();
            if let Some(x) = name {
                let y = x.replace("?", "");
                if let Some(x) = self.message_list.get(&y) {
                    x.hand_msg(self.app.clone(), msg, ctx).await;
                }
            }
        }
    }
}
