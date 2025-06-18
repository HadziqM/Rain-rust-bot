use std::{ops::Deref, sync::Arc};

use appflow::Appflow;
use common::SYSDIR;
use lib_process::App;
use log::debug;
use logger::Mylogger;
use serenity::{all::GatewayIntents, Client};
use setup::DiscordHandler;

pub mod command;
pub mod error;
pub mod macros;
pub mod setup;
pub mod utils;

pub mod all {
    pub use crate::button_reg;
    pub use crate::command_reg;
    pub use crate::modal_reg;
    pub use crate::utils::*;
    pub use std::sync::Arc;
}

pub struct MyApp(App);

impl Deref for MyApp {
    type Target = App;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Appflow for MyApp {
    async fn update_config(self: Arc<Self>) -> appflow::GithubUpdater {
        let setting = self.setting.read().await;
        let update = &setting.main.updater;
        appflow::GithubUpdater {
            repo: update.repo.clone(),
            owner: update.owner.clone(),
            token: update.token.clone(),
            app_name: update.app_name.clone(),
        }
    }

    async fn main_process(self: Arc<Self>) {
        let setting = self.setting.read().await;
        Mylogger::webhook_url(&setting.main.discord.webhook, setting.main.discord.author)
            .set_file_logger(SYSDIR.config_dir("botlog.txt").execute_dir())
            .add_exception("serenity")
            .init();
        log::debug!("Logger initialized");

        let intents = GatewayIntents::GUILDS
            | GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT
            | GatewayIntents::GUILD_MEMBERS;
        let discord = DiscordHandler::new(self.clone());
        let mut client = Client::builder(&setting.main.discord.token, intents)
            .event_handler(discord)
            .await
            .expect("Err creating client");
        if let Err(why) = client.start().await {
            log::error!("Client error: {why:?}");
        }
    }
    async fn cleanup(self: Arc<Self>) {
        debug!("Just sleep 2 sec to emulate clean up");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await
    }
}
#[tokio::main]
async fn main() {
    let app = App::new().await;
    MyApp(app).init().await
}
