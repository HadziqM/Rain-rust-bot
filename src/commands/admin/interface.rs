use serenity::all::{CommandInteraction, ButtonStyle};
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage, CreateEmbed, CreateActionRow};
use serenity::prelude::Context;
use crate::{Init,ErrorLog,Components};
use crate::reusable::utils::color;

pub async fn run(ctx:&Context,cmd:&CommandInteraction,init:&Init){
    let emb = CreateEmbed::new().title("MHFZ user interface")
        .color(color("40", "ff", "40"))
        .description("button interface for mhfz player to make use of server's utility");
    let arow = CreateActionRow::Buttons(
        vec![Components::normal_button("register", "register", ButtonStyle::Primary, "📝"),
             Components::normal_button("DM save", "dms", ButtonStyle::Success, "🔐")]
        );
    if let Err(why) = cmd.create_response(&ctx.http, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
                    .embed(emb).components(vec![arow]))).await{
        let mut err = ErrorLog::new(ctx, init, &cmd.user).await;
        err.discord_error(why.to_string(), "interface command").await;
    }
}
