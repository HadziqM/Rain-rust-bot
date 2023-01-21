pub mod interaction;
pub mod ready;

pub struct Handler;

use serenity::async_trait;
use serenity::model::prelude::Ready;
use serenity::prelude::*;
use serenity::model::application::interaction::Interaction;


#[async_trait]
impl EventHandler for Handler{
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = command.data.name.as_str();
            interaction::slash_command(content, &command,&ctx).await;
        }
    }
    async fn ready(&self, ctx:Context, ready:Ready){
        ready::ready(ctx, ready).await
    }

}
