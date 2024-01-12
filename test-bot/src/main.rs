#![allow(unused)]
use std::collections::HashMap;

use ::serenity::futures::{future::ok, lock::Mutex};
use dotenv::{dotenv, var};
use poise::{serenity_prelude as seren, CreateReply};

struct Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong").await?;
    Ok(())
}
#[poise::command(slash_command, prefix_command)]
async fn error(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("a".parse::<u8>()?.to_string()).await?;
    Ok(())
}

async fn on_error(err: poise::FrameworkError<'_, Data, Error>) {
    match err {
        poise::FrameworkError::Setup { error, .. } => {
            panic!("failed to setup with err messages {error:?}")
        }
        poise::FrameworkError::Command { error, ctx, .. } => {
            ctx.say("this can be error").await;
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv();
    let token = var("TOKEN").expect("cant find token in .env");
    let intents = seren::GatewayIntents::non_privileged() | seren::GatewayIntents::MESSAGE_CONTENT;

    let opt = poise::FrameworkOptions {
        commands: vec![ping(), error()],
        on_error: |err| Box::pin(on_error(err)),
        ..Default::default()
    };
    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(opt)
        .build();

    let client = seren::client::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
