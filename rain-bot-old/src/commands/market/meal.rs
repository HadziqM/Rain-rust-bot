use super::market::Bought;
use crate::reusable::component::bounty::Title;
use crate::reusable::component::market::{Market, Meal, Trading};
use crate::{Components, MyErr, Reg, SlashBundle};
use serde_json::Value;
use serenity::all::*;

pub(super) async fn slash(bnd: &SlashBundle<'_>, reg: &Reg<'_>) -> Result<(), MyErr> {
    let trade = Trading::new().await?;
    if !trade.restourant.enabled {
        return Err(MyErr::Custom(
            "restourant is currently disabled".to_string(),
        ));
    }
    let meal = Meal::new().await?;
    let mut bought = 0;
    let mut ids = 0;
    for data in Components::sub_options(bnd)? {
        if let CommandDataOptionValue::Integer(x) = &data.value {
            if x < &0 {
                return Err(MyErr::Custom(
                    "you cant bought 0 or negative quantity".to_owned(),
                ));
            }
            match data.name.as_str() {
                "food" => ids = x.to_owned(),
                _ => bought = x.to_owned(),
            }
        }
    }
    let meals = meal
        .meals
        .iter()
        .find(|&x| x.id == ids as i32)
        .ok_or(MyErr::Custom(
            "unidentified input, make sure you properly select food options".to_owned(),
        ))?;
    let (co, event) = tokio::join!(reg.pg.get_coin(), reg.pg.get_event());
    let (coin, event) = (co?, event?);
    let title = Title::new(event.title as u8).discount();
    let bon = (1.0 - title) * 100.0;
    let disc = format!("{}%", bon as u32);
    let total = bought * trade.restourant.price as i64;
    let discounted = (total as f32 * title) as i64;
    let change = coin as i64 - discounted;
    if change < 0 {
        return Err(MyErr::Custom(format!(
            "you only have {} bounty coin, and you need {} for this transaction",
            Market::currency(coin as i64),
            Market::currency(total)
        )));
    }
    let exp = crate::reusable::utils::MyTime::elapsed(bought * 360);
    let receipt = Bought::new(
        meals.name.to_string(),
        bought,
        total,
        change,
        coin,
        trade.restourant.price,
        "Hour(s)".to_owned(),
        discounted,
        disc,
    );
    if receipt.confirmation(bnd).await? {
        if !reg
            .pg
            .guild_food(reg.cid, ids as i32, meals.level, exp as i32)
            .await?
        {
            return Err(MyErr::Custom(
                "You dont have any guild to use this command (dont worry your bounty is refounded)"
                    .to_owned(),
            ));
        }
        reg.pg.bounty_transaction(total as i32).await?;
    }
    Ok(())
}

pub(super) async fn auto(bnd: &SlashBundle<'_>, focus: &str) -> Result<(), MyErr> {
    let meal = Meal::new().await?;
    let mut auto = meal
        .meals
        .iter()
        .filter_map(|x| {
            let name = x.name.to_lowercase();
            let foc = focus.to_lowercase();
            if name.starts_with(&foc) || name.contains(&foc) {
                return Some(AutocompleteChoice {
                    value: Value::Number(x.id.into()),
                    name: x.name.to_owned(),
                });
            }
            None
        })
        .collect::<Vec<_>>();
    if auto.len() > 25 {
        auto = auto[..20].to_vec();
    }
    bnd.cmd
        .create_response(
            &bnd.ctx.http,
            CreateInteractionResponse::Autocomplete(
                CreateAutocompleteResponse::new().set_choices(auto),
            ),
        )
        .await?;
    Ok(())
}
