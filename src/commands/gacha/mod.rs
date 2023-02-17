pub mod pull;
pub mod ch_gacha;


use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::all::CommandOptionType;
use crate::AppReg;



pub fn reg()->Vec<CreateCommand>{
    let pull = AppReg::normal_slash("pull", "pull gacha with gacha ticket").add_option(
        CreateCommandOption::new(CommandOptionType::String, "item", "the available items").required(true).set_autocomplete(true)
        ).add_option(
        CreateCommandOption::new(CommandOptionType::Integer, "bought", "the number you want to buy").required(true)
        );
    vec![
        pull
    ]
}
