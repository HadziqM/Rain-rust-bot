use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use crate::AppReg;

pub mod card;
pub mod save;
pub mod transfer;

pub fn reg()->Vec<CreateApplicationCommand>{
    let mut file = CreateApplicationCommand::default();
    file.name("transfer").description("transfer your save data to server (dont send empty file)")
        .create_option(|op|op.name("file0").description("attach your binary")
        .kind(CommandOptionType::Attachment).required(true))
        .create_option(|op|op.name("file1").description("attach your binary")
        .kind(CommandOptionType::Attachment).required(false))
        .create_option(|op|op.name("file2").description("attach your binary")
        .kind(CommandOptionType::Attachment).required(false))
        .create_option(|op|op.name("file3").description("attach your binary")
        .kind(CommandOptionType::Attachment).required(false))
        .create_option(|op|op.name("file4").description("attach your binary")
        .kind(CommandOptionType::Attachment).required(false))
        .create_option(|op|op.name("file5").description("attach your binary")
        .kind(CommandOptionType::Attachment).required(false))
        .create_option(|op|op.name("file6").description("attach your binary")
        .kind(CommandOptionType::Attachment).required(false));
    vec![
        AppReg::normal_slash("card", "show your hunter card status"),
        AppReg::user_context("👤 Card"),
        AppReg::normal_slash("dm_save", "make bot send you your save file"),
        file
    ]
}
