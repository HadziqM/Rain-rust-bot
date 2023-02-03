use serenity::{builder::{CreateCommand, CreateCommandOption}, all::CommandOptionType};

use crate::AppReg;

pub mod create;
pub mod bind;
pub mod check;
pub mod change_pasword;

pub fn reg()->Vec<CreateCommand>{
    let mut out = Vec::new();
    out.push(AppReg::normal_slash("create", "create mhfz accont to this server"));
    out.push(AppReg::normal_slash("check", "check your user data in server"));
    out.push(AppReg::normal_slash("switch", "switch your character for discord event"));
    let pass = AppReg::normal_slash("change_pasword", "change your account password")
        .add_option(CreateCommandOption::new(CommandOptionType::String, "password", "your new password"));
    out.push(pass);
    out
}
