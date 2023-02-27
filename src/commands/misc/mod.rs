pub mod ferias;
pub mod tag;

use crate::AppReg;
use serenity::all::*;

pub fn reg()->Vec<CreateCommand>{
    let ferias = AppReg::normal_slash("ferias", "link you a ferias item").add_option(CreateCommandOption::new(CommandOptionType::String, "item", "the item you wanna search").set_autocomplete(true).required(true));
    vec![ferias]
}
