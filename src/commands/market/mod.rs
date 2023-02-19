pub mod market;
pub mod ch_market;

use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::all::CommandOptionType;
use crate::AppReg;



pub fn reg()->Vec<CreateCommand>{
    let stall = AppReg::normal_slash("stall", "buy from market stall").add_option(
        CreateCommandOption::new(CommandOptionType::String, "item", "item name").set_autocomplete(true).required(true)
        ).add_option(
        CreateCommandOption::new(CommandOptionType::Integer, "quantity", "number of item you bought").required(true)
        );
    vec![
        stall
    ]
}

