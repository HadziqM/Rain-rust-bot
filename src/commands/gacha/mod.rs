pub mod pull;
pub mod ch_gacha;


use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::all::CommandOptionType;
use crate::AppReg;



pub fn reg()->Vec<CreateCommand>{
    let pull = AppReg::normal_slash("pull", "pull gacha with gacha ticket").add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "single", "single gacha pull cost 10 ticket")
        ).add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "multi", "12 gacha pull cost 110 ticket")
        );
    let ch = AppReg::admin_slash("ch_gacha", "cahnge gacha banner with json").add_option(
        CreateCommandOption::new(CommandOptionType::Attachment, "attachment", "you gacha.json file (need to be correct name)").required(true)
        );
    vec![
        pull,ch
    ]
}

