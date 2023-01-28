use serenity::{builder::CreateApplicationCommand, model::prelude::command::CommandOptionType};
use crate::AppReg;


pub mod interface;
pub mod save_cd;


pub fn reg()->Vec<CreateApplicationCommand>{
    let mut save = AppReg::admin_slash("reset_save_cd", "reset someone save cooldown");
    save.create_option(|op|op.name("user").description("mention user you want to reset").kind(CommandOptionType::User).required(true));
    vec![
        AppReg::admin_slash("interface", "mhfz user interface"),
        save
    ]
}
