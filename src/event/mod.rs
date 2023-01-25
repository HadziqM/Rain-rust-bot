pub mod interaction;
pub mod ready;


use serenity::async_trait;
use serenity::model::prelude::Ready;
use serenity::prelude::*;
use serenity::model::application::interaction::{Interaction::*,Interaction};
use crate::reusable::config::*;
use interaction::*;


#[derive(Debug,Clone)]
pub struct Handler{
    pub config:Init
}

#[async_trait]
impl EventHandler for Handler{
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let init = self.config.clone();
        match interaction {
            ModalSubmit(mut cmd)=>modal_command(&cmd.data.to_owned().custom_id, &mut cmd, &ctx,&init).await,
            ApplicationCommand(cmd) => slash_command(&cmd.data.name, &cmd,&ctx, &init).await,
            MessageComponent(cmd) => button_command(&cmd.data.custom_id, &cmd,&ctx, &init).await,
            _=>println!("unhandled interaction")
        }
    }
    async fn ready(&self, ctx:Context, ready:Ready){
        ready::ready(ctx, ready).await
    }

}
