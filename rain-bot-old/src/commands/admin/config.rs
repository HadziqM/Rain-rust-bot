use crate::reusable::component::{
    bounty::{BountyRefresh, BountyTitle, Category},
    gacha::Gacha,
    market::{Market, Meal, Tag, Trading},
};
use crate::{Components, MyErr, Mybundle, Mytrait, SlashBundle};
use hertz::hertz_slash_normal;
use serenity::all::*;

#[hertz_slash_normal(0, true)]
async fn slash(bnd: &SlashBundle<'_>) -> Result<(), MyErr> {
    let mut name = "";
    for data in &bnd.cmd.data.options {
        if let CommandDataOptionValue::SubCommand(_) = &data.value {
            name = &data.name;
        }
    }
    match name {
        "gacha" => Components::json_config(bnd, Gacha::default()).await?,
        "market" => Components::json_config(bnd, Market::default()).await?,
        "bounty" => bounty(bnd).await?,
        "bounty_refresh" => Components::json_config(bnd, BountyRefresh::default()).await?,
        "bounty_title" => Components::json_config(bnd, BountyTitle::default()).await?,
        "meal" => Components::json_config(bnd, Meal::default()).await?,
        "trading" => Components::json_config(bnd, Trading::default()).await?,
        "tag" => Components::json_config(bnd, Tag::default()).await?,
        _ => {
            return Err(MyErr::Custom(
                "you dont have any configuration needed to change".to_owned(),
            ))
        }
    };
    Components::edit(bnd, "success").await?;
    Ok(())
}

async fn bounty(bnd: &SlashBundle<'_>) -> Result<(), MyErr> {
    let mut x = "";
    for i in Components::sub_options(bnd)? {
        if let CommandDataOptionValue::String(y) = &i.value {
            x = &y
        }
    }
    let category = Category::new(x.parse::<u8>().unwrap())?;
    Components::bounty_config(bnd, &category).await?;
    Ok(())
}
