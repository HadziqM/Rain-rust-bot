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
        if let Interaction::ApplicationCommand(command) = interaction.clone() {
            let content = command.data.name.as_str();
            interaction::slash_command(content, &command,&ctx).await;
        };
        if let Interaction::MessageComponent(command) = interaction.clone() {
            let cid = command.data.custom_id.as_str();
            interaction::button_command(cid, &command, &ctx).await;

        };
        if let Interaction::ModalSubmit(command) = interaction {
            let cid = command.data.custom_id.as_str();
            interaction::modal_command(cid, &command, &ctx).await;
        }
    }
    async fn ready(&self, ctx:Context, ready:Ready){
        ready::ready(ctx, ready).await
    }

}
