pub mod config;
pub mod error;
pub mod utils;

use std::sync::Arc;
use error::MyErr;
use config::Init;
use binding::{postgres::Db, bounty::BountyGlobal};
use material::ItemPedia;
use poise::serenity_prelude as serenity;


pub struct AppData {
    pub init: Init,
    pub db: Db,
    pub bounty: Arc<BountyGlobal>,
    pub pedia: ItemPedia
}


pub type Context<'a> = poise::Context<'a,AppData,MyErr>;


#[tokio::main]
async fn main() -> Result<(),MyErr> {
    let init = Init::new().await?;
    let app = AppData {
        init: init.clone(),
        db : Db::connect(&init.postgress).await?,
        pedia: ItemPedia::default(),
        bounty : BountyGlobal::create(),
    };
    let intents = serenity::GatewayIntents::GUILDS | serenity::GatewayIntents::GUILD_MESSAGES | serenity::GatewayIntents::GUILD_MEMBERS | serenity::GatewayIntents::MESSAGE_CONTENT | serenity::GatewayIntents::non_privileged();

    let options:poise::FrameworkOptions<AppData, MyErr> = poise::FrameworkOptions::default();
    let framework = poise::Framework::builder()
        .setup(move |ctx,_ready,framework|{
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(app)
            })
        })
        .options(options)
        .build();
    let client = serenity::ClientBuilder::new(&init.discord.token,intents)
        .framework(framework).await;
    client?.start().await?;
    Ok(())
}
