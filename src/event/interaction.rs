use serenity::{async_trait, model::prelude::GuildId};
use serenity::model::prelude::{Ready, Activity};
use serenity::prelude::*;
use crate::commands;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};

use super::Handler;

#[async_trait]
impl EventHandler for Handler{
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "id" => commands::id::run(&command.data.options),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
    async fn ready(&self, ctx:Context, ready:Ready){
        let user = ready.user.name;
        println!("{} is running", &user);
        for guild in &ready.guilds{
            println!("{} is on guild **{}**",&user,guild.id);
            GuildId::set_application_commands(&guild.id, &ctx.http, |apps|{
                apps
                    .create_application_command(|command| commands::id::register(command))
                    .create_application_command(|command| commands::ping::register(command))
            }).await.unwrap();
        }
        ctx.set_activity(Activity::competing("i want to die")).await;
    }

}
