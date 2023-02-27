pub mod market;
pub mod trading;
pub mod meal;

use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::all::CommandOptionType;
use crate::AppReg;



pub fn reg()->Vec<CreateCommand>{
    let market = CreateCommandOption::new(CommandOptionType::SubCommand,"market","server market trading center").add_sub_option(
        CreateCommandOption::new(CommandOptionType::String, "item", "item name").set_autocomplete(true).required(true)
        ).add_sub_option(
        CreateCommandOption::new(CommandOptionType::Integer, "quantity", "number of item you bought").required(true)
        );
    let bar = CreateCommandOption::new(CommandOptionType::SubCommand,"bar","the trading bar to request custom item with bartender");
    let restourant = CreateCommandOption::new(CommandOptionType::SubCommand,"restourant","restourant to order guildfood").add_sub_option(
        CreateCommandOption::new(CommandOptionType::Integer, "food", "the food you order").set_autocomplete(true).required(true)
        ).add_sub_option(
        CreateCommandOption::new(CommandOptionType::Integer, "duration", "the food duration in hour").required(true)
        );
    let casino = CreateCommandOption::new(CommandOptionType::SubCommand,"casino","casino to exchange gacha ticket").add_sub_option(
        CreateCommandOption::new(CommandOptionType::Integer, "ticket", "the amount of ticket you want to bought").required(true)
        );
    let guild = CreateCommandOption::new(CommandOptionType::SubCommand,"guild","guild trade to buy guild rp").add_sub_option(
        CreateCommandOption::new(CommandOptionType::Integer, "rp", "the amount of rp you want to bought").required(true)
        );
    let jawelry = CreateCommandOption::new(CommandOptionType::SubCommand,"jewelry","jewelry to exchange gacha premium").add_sub_option(
        CreateCommandOption::new(CommandOptionType::Integer, "amount", "the amount of point you want to bought").required(true)
        );
    let stall = AppReg::normal_slash("trading", "server trading center to get the item you want").add_option(market)
        .add_option(bar).add_option(restourant).add_option(casino).add_option(jawelry).add_option(guild);
    vec![
        stall
    ]
}
