use serenity::builder::CreateApplicationCommand;
use crate::AppReg;


pub mod interface;


pub fn reg()->Vec<CreateApplicationCommand>{
    vec![
        AppReg::admin_slash("interface", "mhfz user interface")
    ]
}
