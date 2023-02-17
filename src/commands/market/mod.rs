pub mod market;
pub mod ch_market;

use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::all::CommandOptionType;
use crate::AppReg;



pub fn reg()->Vec<CreateCommand>{
    let stall = AppReg::normal_slash("stall", "buy from market stall").add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "single", "single gacha pull cost 10 ticket")
        ).add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "multi", "12 gacha pull cost 110 ticket")
        );
    vec![
        stall
    ]
}

