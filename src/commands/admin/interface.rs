use serenity::all::ButtonStyle;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage, CreateEmbed, CreateActionRow};
use crate::{Components,SlashBundle,MyErr};
use crate::reusable::utils::color;

pub async fn slash(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    let mut button = Vec::from(
        [Components::normal_button("register", "register", ButtonStyle::Primary, "📝"),
        Components::normal_button("DM save", "dms", ButtonStyle::Success, "🔐")]
        );
    if !bnd.init.mhfz_config.account_creation{
        button.push(
            Components::normal_button("Bind", "bind", ButtonStyle::Secondary, "🎀")
            )
    }
    let emb = CreateEmbed::new().title("MHFZ user interface")
        .color(color("40", "ff", "40"))
        .description("button interface for mhfz player to make use of server's utility");
    let arow = CreateActionRow::Buttons(button);
    let resp = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().embed(emb).components(vec![arow]));
    Components::response_adv(bnd, resp).await?;
    Ok(())
}
