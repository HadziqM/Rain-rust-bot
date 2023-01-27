use serenity::{builder::CreateApplicationCommand, model::prelude::command::CommandOptionType};

use crate::AppReg;

pub mod interface;
pub mod create;
pub mod bind;
pub mod check;
pub mod change_pasword;

pub fn reg()->Vec<CreateApplicationCommand>{
    let mut out = Vec::new();
    out.push(AppReg::normal_slash("interface", "mhfz user interface"));
    out.push(AppReg::normal_slash("create", "create mhfz accont to this server"));
    out.push(AppReg::normal_slash("check", "check your user data in server"));
    out.push(AppReg::normal_slash("switch", "switch your character for discord event"));
    let mut pass = CreateApplicationCommand::default();
    pass.name("change_password").description("change your account password")
        .create_option(|op|op.name("password").description("your new password")
        .kind(CommandOptionType::String).required(true));
    out.push(pass);
    out
}
