use serenity::builder::CreateApplicationCommand;
use crate::AppReg;

pub mod card;
pub mod save;

pub fn reg()->Vec<CreateApplicationCommand>{
    vec![
        AppReg::normal_slash("card", "show your hunter card status"),
        AppReg::user_context("Card"),
        AppReg::normal_slash("dm_save", "make bot send you your save file")
    ]
}
