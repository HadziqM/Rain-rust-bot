use serenity::{builder::{CreateCommand, CreateCommandOption}, all::CommandOptionType};

use crate::AppReg;

pub mod create;
pub mod change_pasword;

pub fn reg()->Vec<CreateCommand>{
    let mut out = Vec::new();
    out.push(AppReg::normal_slash("create", "create mhfz account to this server"));
    out.push(AppReg::normal_slash("bind", "bind existing rain server mhfz account to discord"));
    out.push(AppReg::normal_slash("check", "check your user data in server"));
    out.push(AppReg::normal_slash("switch", "switch your character for discord event"));
    let pass = AppReg::normal_slash("change_password", "change your account password")
        .add_option(CreateCommandOption::new(CommandOptionType::String, "password", "your new password").required(true));
    let psn = AppReg::normal_slash("add_psn", "update your psn id to your registered account")
        .add_option(CreateCommandOption::new(CommandOptionType::String, "psn_id", "your psn id").required(true));
    out.push(pass);
    out.push(psn);
    out
}
