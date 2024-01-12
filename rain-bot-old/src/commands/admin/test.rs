use crate::reusable::component::bounty::BountyTitle;
use crate::reusable::component::Components;
use crate::{MyErr, Mybundle, Mytrait, SlashBundle};
use serde_json::Value;
use serenity::all::*;

#[hertz::hertz_auto]
async fn auto(bnd: &SlashBundle<'_>) -> Result<(), MyErr> {
    let mut foc = "";
    for i in Components::sub_options(bnd)? {
        if let CommandDataOptionValue::Autocomplete { kind: _, value } = &i.value {
            foc = value.as_str();
        }
    }
    let title = BountyTitle::new().await?;
    let mut option = Vec::new();
    for i in title.custom {
        let opt = AutocompleteChoice {
            name: format!("Trigger on {}", BountyTitle::name(&i.trigger)),
            value: Value::String(i.trigger),
        };
        option.push(opt);
    }
    option.push(AutocompleteChoice {
        name: format!(
            "Trigger on {}",
            BountyTitle::name(&title.bronze_bounty.trigger)
        ),
        value: Value::String(title.bronze_bounty.trigger),
    });
    option.push(AutocompleteChoice {
        name: format!(
            "Trigger on {}",
            BountyTitle::name(&title.silver_bounty.trigger)
        ),
        value: Value::String(title.silver_bounty.trigger),
    });
    option.push(AutocompleteChoice {
        name: format!(
            "Trigger on {}",
            BountyTitle::name(&title.gold_bounty.trigger)
        ),
        value: Value::String(title.gold_bounty.trigger),
    });
    option.push(AutocompleteChoice {
        name: format!(
            "Trigger on {}",
            BountyTitle::name(&title.bronze_trading.trigger)
        ),
        value: Value::String(title.bronze_trading.trigger),
    });
    option.push(AutocompleteChoice {
        name: format!(
            "Trigger on {}",
            BountyTitle::name(&title.silver_trading.trigger)
        ),
        value: Value::String(title.silver_trading.trigger),
    });
    option.push(AutocompleteChoice {
        name: format!(
            "Trigger on {}",
            BountyTitle::name(&title.gold_trading.trigger)
        ),
        value: Value::String(title.gold_trading.trigger),
    });
    let mut filtered = option
        .iter()
        .filter(|x| x.name.contains(foc))
        .map(|x| x.to_owned())
        .collect::<Vec<_>>();
    if filtered.len() > 20 {
        filtered = filtered[..20].to_vec();
    }
    bnd.cmd
        .create_response(
            &bnd.ctx.http,
            CreateInteractionResponse::Autocomplete(
                CreateAutocompleteResponse::new().set_choices(filtered),
            ),
        )
        .await?;
    Ok(())
}

#[hertz::hertz_slash_normal(0, true)]
async fn slash(bnd: &SlashBundle<'_>) -> Result<(), MyErr> {
    let mut item = "";
    for i in &bnd.cmd.data.options {
        if let CommandDataOptionValue::SubCommand(_) = &i.value {
            item = i.name.as_str();
        }
    }
    match item {
        "test" => test(bnd).await?,
        _ => gift(bnd).await?,
    }
    Ok(())
}
async fn test(bnd: &SlashBundle<'_>) -> Result<(), MyErr> {
    let mut item = "";
    for i in Components::sub_options(bnd)? {
        if let CommandDataOptionValue::String(x) = &i.value {
            item = x.as_str();
        }
    }
    let title = BountyTitle::new().await?;
    let images = title.hash();
    let image = images.get(item).ok_or(MyErr::Custom(
        "cant get option value, make sure properly select option".to_owned(),
    ))?;
    let att = CreateAttachment::bytes(
        image
            .image
            .title(
                bnd.cmd
                    .user
                    .static_avatar_url()
                    .unwrap_or(bnd.cmd.user.default_avatar_url())
                    .as_str(),
            )
            .await?,
        "title.png",
    );
    bnd.cmd
        .edit_response(
            &bnd.ctx.http,
            EditInteractionResponse::new().new_attachment(att),
        )
        .await?;
    Ok(())
}

async fn gift(bnd: &SlashBundle<'_>) -> Result<(), MyErr> {
    let mut item = "";
    let user = bnd.cmd.data.resolved.users.values().next().unwrap();
    for i in Components::sub_options(bnd)? {
        if let CommandDataOptionValue::String(x) = &i.value {
            item = x.as_str();
        }
    }
    let mut mem = bnd
        .cmd
        .guild_id
        .unwrap()
        .member(&bnd.ctx.http, &user.id)
        .await?;
    let mut pg = crate::PgConn::create(bnd.init, user.id.to_string()).await?;
    let mut event = pg.get_event().await?;
    let title = BountyTitle::new().await?;
    title.add_title(bnd, &mut event, &mut mem, item).await?;
    Components::response(bnd, "success", true).await?;
    pg.bounty_event(&event).await?;
    pg.close().await;
    Ok(())
}
