pub mod pull;


use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::all::CommandOptionType;
use crate::AppReg;



pub fn reg()->Vec<CreateCommand>{
    let pull = AppReg::normal_slash("pull", "pull gacha with gacha ticket").add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "single", "single gacha pull cost 10 ticket")
        ).add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "multi", "12 gacha pull cost 110 ticket")
        );
    vec![
        pull
    ]
}

