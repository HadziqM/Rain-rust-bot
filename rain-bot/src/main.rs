use std::sync::Arc;

use appflow::Appflow;
use common::{setting::SettingAll, SYSDIR};
use logger::Mylogger;
use serenity::{all::GatewayIntents, Client};
use setup::{App, DiscordHandler};
use tokio::sync::RwLock;

pub mod error;
pub mod setup;

impl Appflow for App {
    async fn main_process(self: Arc<Self>) {
        let setting = self.setting.read().await;
        Mylogger::webhook_url(&setting.main.discord.webhook, setting.main.discord.author)
            .set_file_logger(SYSDIR.log_dir("botlog.txt").execute_dir())
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
            log::error!("Client error: {:?}", why);
        }
    }
    async fn cleanup(self: Arc<Self>) {}
}
#[tokio::main]
async fn main() {
    let setting = SettingAll::load_all();
    let app = App {
        setting: Arc::new(RwLock::new(setting)),
    };
    app.init().await
}
