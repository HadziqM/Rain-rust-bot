use crate::{Components, ModalBundle, MyErr, Mybundle, Mytrait, Reg, SlashBundle};
use serenity::all::*;

#[hertz::hertz_slash_normal(60, false)]
async fn slash(bnd: &SlashBundle<'_>) -> Result<(), MyErr> {
    let mut reg = Reg::only_check_alter(bnd, &bnd.cmd.user).await?;
    if let CommandDataOptionValue::String(pass) = &bnd.cmd.data.options.first().unwrap().value {
        reg.pg.change_user_password(pass.as_str(), reg.cid).await?;
        bnd.cmd
            .create_response(
                &bnd.ctx.http,
                Components::interaction_response("your password succesfully changed", true),
            )
            .await?;
    }
    reg.pg.close().await;
    Ok(())
}

//psn_id
#[hertz::hertz_combine_normal(60, false)]
async fn all<T: Mybundle>(bnd: &T) -> Result<(), MyErr> {
    let res = CreateInteractionResponse::Modal(
        CreateModal::new("add_psn", "Register/Override PSN ID").components(vec![
            CreateActionRow::InputText(
                CreateInputText::new(InputTextStyle::Short, "PSN_ID", "psn_id")
                    .required(true)
                    .placeholder("For console player (You can Leave it Empty)"),
            ),
        ]),
    );
    Components::response_adv(bnd, res).await?;
    Ok(())
}

#[hertz::hertz_modal_normal(60, false)]
async fn psn(bnd: &ModalBundle<'_>) -> Result<(), MyErr> {
    let mut reg = Reg::only_check_alter(bnd, &bnd.cmd.user).await?;
    for comp in &bnd.cmd.data.components {
        let arow = comp.components.first().unwrap();
        if let ActionRowComponent::InputText(input) = arow {
            reg.pg.psn(input.value.as_str(), reg.cid).await?;
            bnd.cmd
                .create_response(
                    &bnd.ctx.http,
                    Components::interaction_response(
                        "your psn id is succesfully registered in your game account",
                        true,
                    ),
                )
                .await?;
        }
    }
    reg.pg.close().await;
    Ok(())
}
#[hertz::hertz_slash_normal(0, false)]
async fn check(bnd: &SlashBundle<'_>) -> Result<(), MyErr> {
    if let Some(mut reg) = Reg::reverse_check(bnd, &bnd.cmd.user).await? {
        let user = reg.pg.get_user().await?;
        Components::response(bnd, &format!("Your Username is = {}", user.1), true).await?;
        reg.pg.close().await;
    };
    Ok(())
}
