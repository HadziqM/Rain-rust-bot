pub mod pull;
pub mod ch_gacha;


use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::all::CommandOptionType;
use crate::AppReg;



pub fn reg()->Vec<CreateCommand>{
    let pull = AppReg::normal_slash("pull", "pull gacha with gacha ticket").add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "single", "single pull with 10 ticket")
        ).add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "multi", "12 pull with 110 ticket")
        );
    vec![
        pull
    ]
}
