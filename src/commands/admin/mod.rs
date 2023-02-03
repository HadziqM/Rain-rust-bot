use serenity::builder::{CreateCommand, CreateCommandOption};

use crate::AppReg;


pub mod interface;
pub mod save_cd;


pub fn reg()->Vec<CreateCommand>{
    let mut save = AppReg::admin_slash("reset_save_cd", "reset someone save cooldown");
    let option = CreateCommandOption::new(serenity::all::CommandOptionType::User,"user","mention user you want to reset").required(true);
    save.add_option(option);
    vec![
        AppReg::admin_slash("interface", "mhfz user interface"),
        save
    ]
}
