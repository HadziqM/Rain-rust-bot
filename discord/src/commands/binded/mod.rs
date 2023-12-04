use serenity::{builder::{CreateCommand, CreateCommandOption}, all::CommandOptionType};

use crate::AppReg;

pub mod card;
pub mod save;
pub mod transfer;
pub mod event;

pub fn reg()->Vec<CreateCommand>{
    let file = AppReg::normal_slash("transfer", "transfer your save data to server")
        .add_option(CreateCommandOption::new(CommandOptionType::Attachment,"file", "attach your savefile .bin").required(true))
        .add_option(CreateCommandOption::new(CommandOptionType::Attachment,"file1", "attach your savefile .bin"))
        .add_option(CreateCommandOption::new(CommandOptionType::Attachment,"file2", "attach your savefile .bin"))
        .add_option(CreateCommandOption::new(CommandOptionType::Attachment,"file3", "attach your savefile .bin"))
        .add_option(CreateCommandOption::new(CommandOptionType::Attachment,"file4", "attach your savefile .bin"))
        .add_option(CreateCommandOption::new(CommandOptionType::Attachment,"file5", "attach your savefile .bin"))
        .add_option(CreateCommandOption::new(CommandOptionType::Attachment,"file6", "attach your savefile .bin"))
        .add_option(CreateCommandOption::new(CommandOptionType::Attachment,"file7", "attach your savefile .bin"))
        .add_option(CreateCommandOption::new(CommandOptionType::Attachment,"file8", "attach your savefile .bin"))
        .add_option(CreateCommandOption::new(CommandOptionType::Attachment,"file9", "attach your savefile .bin"));
    vec![
        AppReg::normal_slash("card", "show your hunter card status"),
        AppReg::user_context("👤 Card"),
        AppReg::user_context("🎀 Event"),
        AppReg::normal_slash("event","show my event data"),
        AppReg::normal_slash("dm_save", "make bot send you your save file"),
        file
    ]
}
