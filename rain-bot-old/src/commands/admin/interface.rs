use std::num::NonZeroU64;

use crate::event::interaction::ComponentBundle;
use crate::reusable::component::registered::Reg;
use crate::reusable::utils::color;
use crate::{Components, MyErr, Mybundle, Mytrait, SlashBundle};
use serenity::all::{ButtonStyle, Colour, RoleId};
use serenity::builder::{
    CreateActionRow, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage,
};

#[hertz::hertz_slash_normal(0, false)]
async fn slash(bnd: &SlashBundle<'_>) -> Result<(), MyErr> {
    let mut button = Vec::from([
        Components::normal_button("Register", "register", ButtonStyle::Primary, "📝"),
        Components::normal_button("Link PSN", "link_psn", ButtonStyle::Primary, "📱"),
    ]);
    if !bnd.init.mhfz_config.account_creation {
        button.push(Components::normal_button(
            "Bind",
            "bind",
            ButtonStyle::Primary,
            "🎀",
        ))
    }
    let button_2 = Vec::from([
        Components::normal_button("DM save", "dms", ButtonStyle::Success, "🔐"),
        Components::normal_button("Claim Role", "claim_role", ButtonStyle::Success, "😀"),
        Components::normal_button(
            "Request Assistance",
            "assistance",
            ButtonStyle::Secondary,
            "🫴",
        ),
    ]);
    let emb = CreateEmbed::new().title("MHFZ Button Interface ")
        .color(color("40", "ff", "40"))
        .description("1. `Register` to create mhfz game account as alternative to using launcher\n2. `Bind` to bind the existing game account to discord\n3. `DM save` to backup your save data if you `fully binded` your account\n4. `Link PSN` to register your console PSN id to your game account or overriding existing one\n5. `Claim Role` to claim missing register role if you ever miss it\n6. `Request Assistance` to request assistance for your problem about the game technical problems");
    let emb2 = CreateEmbed::new()
        .title("Creating New Game Account")
        .description("You can create account from launcher or using the button to create the account, to play the game, simply use the username and password you just created to fill the ID and passowrd on launcher or console, dont worry about any other step just play the game")
        .color(Colour::KERBAL);
    let emb3 = CreateEmbed::new().title("Fully Binded Account").description("To be abble to use discord event and backup or transfer your data you need to be fully bind your game account.\n\n to fully bind your game account you require some thing\n\n 1. You are playing in rain server, we cant do anything if you play elsewhere\n2. Have one or some working characters in your game account, this not to be confused by placeholder character, you need atleast one character that have been in mezeporta after undergoin character customization \n\n If you have fulfilled the requirement you can use following step depending on how you create your game account before\n Scenario 1. `You create your account from discord bot or upcoming website`, you already `half binded` for the get go, you can use any command like `/card`,`/event`,`/pull`,`/switch`,`/transfer`,`/switch` any command will prompt you to select your character if you still `half binded` and you automatically fully binded\n Scenario 2.`You create account from launcher` you need to use `Bind` button bellow and youare now `half binded`, to get fully binded use step scenaroi 1 above after you have working character.").color(Colour::FOOYOO);
    let arow = CreateActionRow::Buttons(button);
    let arow2 = CreateActionRow::Buttons(button_2);
    let resp = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .embeds(vec![emb, emb2, emb3])
            .components(vec![arow, arow2]),
    );
    Components::response_adv(bnd, resp).await?;
    Ok(())
}

//claim_role
#[hertz::hertz_button_normal(0, false)]
async fn claim(bnd: &ComponentBundle<'_>) -> Result<(), MyErr> {
    let mut reg = Reg::check(bnd, &bnd.cmd.user).await?;
    let rid = RoleId(NonZeroU64::new(bnd.init.server_role.register_role).unwrap());
    let mut user = bnd.cmd.member.clone().unwrap();
    user.add_role(&bnd.ctx.http, rid).await?;
    Components::response(bnd, "Role registered already added", true).await?;
    reg.pg.close().await;
    Ok(())
}
//Request assistance
#[hertz::hertz_button_normal(0, false)]
async fn request(bnd: &ComponentBundle<'_>) -> Result<(), MyErr> {
    Components::response(bnd, "Command Still on the development", true).await?;
    Ok(())
}
