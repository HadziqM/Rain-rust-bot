use serenity::builder::{CreateCommand, CreateCommandOption};

use crate::AppReg;


pub mod interface;
pub mod save_cd;


pub fn reg()->Vec<CreateCommand>{
    let save = AppReg::admin_slash("reset_save_cd", "reset someone save cooldown").add_option(
        CreateCommandOption::new(serenity::all::CommandOptionType::User,"user","mention user you want to reset").required(true)
        );
    vec![
        AppReg::admin_slash("interface", "mhfz user interface"),
        save
    ]
}
