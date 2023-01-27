use serenity::builder::CreateApplicationCommand;
use crate::AppReg;

pub mod card;


pub fn reg()->Vec<CreateApplicationCommand>{
    vec![
        AppReg::normal_slash("card", "show your hunter card status"),
        AppReg::user_context("Card")
    ]
}
