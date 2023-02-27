use crate::{MyErr,SlashBundle,Mybundle,Mytrait,Components};
use hertz::hertz_slash_normal;
use serenity::all::*;
use crate::reusable::component::{market::{Market,Meal,Trading},gacha::Gacha,bounty::{BountyRefresh,Bounty}};

#[hertz_slash_normal(0,true)]
async fn slash(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    let mut name = "";
    for data in &bnd.cmd.data.options{
        if let CommandDataOptionValue::SubCommand(_) = &data.value{
            name = &data.name;
        }
    }
    match name{
        "gacha"=>Components::json_config(bnd, Gacha::default()).await?,
        "market"=>Components::json_config(bnd, Market::default()).await?,
        "bounty"=>Components::json_config(bnd, Bounty::default()).await?,
        "bounty_refresh"=>Components::json_config(bnd, BountyRefresh::default()).await?,
        "meal"=>Components::json_config(bnd, Meal::default()).await?,
        "trading"=>Components::json_config(bnd,Trading::default()).await?,
        _=>{return Err(MyErr::Custom("you dont have any configuration needed to change".to_owned()))}
    };
    Ok(())
}
