use crate::{Init, MyErr};
use ::serenity::{all::UserId, gateway::ActivityData};
use binding::{bounty::BountyGlobal, postgres::Db};
use material::ItemPedia;
use poise::serenity_prelude as serenity;
use poise::{Command, Framework, FrameworkOptions};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

#[derive(Clone)]
pub struct AppData {
    pub init: Arc<RwLock<Init>>,
    pub db: Db,
    pub bounty: Arc<BountyGlobal>,
    pub pedia: ItemPedia,
}

pub type Context<'a> = poise::Context<'a, AppData, MyErr>;
pub type ApplicationContext<'a> = poise::ApplicationContext<'a, AppData, MyErr>;

pub struct Setup {
    framework: Framework<AppData, MyErr>,
    token: String,
}

impl AppData {
    async fn new() -> Result<Self, MyErr> {
        let init = Arc::new(RwLock::new(Init::new().await?));
        let mine = Self {
            init: init.clone(),
            db: Db::connect(&init.read().await.postgress).await?,
            pedia: ItemPedia::default(),
            bounty: BountyGlobal::create().await,
        };
        Ok(mine)
    }
    async fn autosave(&self) -> JoinHandle<()> {
        let bounty = self.bounty.clone();
        tokio::spawn(Box::pin(async move {
            tokio::signal::ctrl_c().await.unwrap();
            println!("\nTrying to Shutdown Gracefully");
            if let Err(why) = bounty.caching().await {
                println!("cant save cache object: {why}")
            }
            std::process::exit(0);
        }))
    }
}

impl Setup {
    fn option(commands: Vec<Command<AppData, MyErr>>) -> FrameworkOptions<AppData, MyErr> {
        FrameworkOptions {
            commands,
            on_error: |err| Box::pin(MyErr::on_error(err)),
            ..Default::default()
        }
    }
    pub async fn new(commands: Vec<Command<AppData, MyErr>>) -> Result<Self, MyErr> {
        let data = AppData::new().await?;
        let token = data.init.read().await.discord.token.to_owned();
        Ok(Setup {
            framework: Framework::builder()
                .setup(move |ctx, ready, framework| {
                    Box::pin(async move {
                        let user = UserId::new(data.init.read().await.discord.author_id)
                            .to_user(&ctx.http)
                            .await?;
                        println!(
                            "----------------------------------------------------------------"
                        );
                        println!(
                            "-------------------------- START -------------------------------"
                        );
                        println!(
                            "----------------------------------------------------------------"
                        );
                        println!("🤖 Bot is running as {}", ready.user.name);
                        println!("🛠 {} is acknowledged as author", user.tag());
                        for guild in &ready.guilds {
                            let partial = guild.id.to_partial_guild(&ctx.http).await?;
                            println!("🏛 {} is on guild **{}**", &ready.user.name, &partial.name);
                        }
                        println!(
                            "----------------------------------------------------------------"
                        );
                        println!(
                            "----------------------- Command List ---------------------------"
                        );
                        println!(
                            "----------------------------------------------------------------"
                        );
                        for comm in &framework.options().commands {
                            println!(
                                "name :{},\tident:{},\taliases:{:?}",
                                comm.name, comm.identifying_name, comm.aliases
                            );
                        }
                        poise::builtins::register_globally(ctx, &framework.options().commands)
                            .await?;
                        ctx.set_activity(Some(ActivityData::playing("With your life")));
                        println!("All Configured Nicely");
                        let _ = data.autosave().await;
                        Ok(data)
                    })
                })
                .options(Self::option(commands))
                .build(),
            token,
        })
    }

    pub async fn run(self) {
        let intents = serenity::GatewayIntents::GUILDS
            | serenity::GatewayIntents::GUILD_MESSAGES
            | serenity::GatewayIntents::GUILD_MEMBERS
            | serenity::GatewayIntents::MESSAGE_CONTENT
            | serenity::GatewayIntents::non_privileged();
        let client = serenity::ClientBuilder::new(&self.token, intents)
            .framework(self.framework)
            .await;
        client.unwrap().start().await.unwrap()
    }
}
