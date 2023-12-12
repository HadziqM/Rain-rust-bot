#![allow(unused)]
use dotenv::{dotenv,var};
use poise::{serenity_prelude as serenity, CreateReply};
use ::serenity::futures::future::ok;

struct Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a,Data,Error>;

#[poise::command(slash_command,prefix_command)]
async fn ping(ctx:Context<'_>) -> Result<(),Error> {
    ctx.say("pong").await?;
    Ok(())
}
#[poise::command(slash_command,prefix_command)]
async fn error(ctx:Context<'_>) -> Result<(),Error> {
    ctx.say("a".parse::<u8>()?.to_string()).await?;
    Ok(())
}

async fn on_error(err:poise::FrameworkError<'_,Data,Error>) {
    match err {
        poise::FrameworkError::Setup { error, .. } => {
            panic!("failed to setup with err messages {error:?}")
        }
        poise::FrameworkError::Command { error, ctx } => {
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
    let token  = var("TOKEN")
        .expect("cant find token in .env");

    let opt = poise::FrameworkOptions {
        commands:vec![ping(),error()],
        on_error: |err| Box::pin(on_error(err)),
        ..Default::default()
    };
}
