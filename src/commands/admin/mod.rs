use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::all::CommandOptionType;
use crate::AppReg;


pub mod interface;
pub mod save_cd;
pub mod market;


pub fn reg()->Vec<CreateCommand>{
    let save = AppReg::admin_slash("reset_save_cd", "reset someone save cooldown").add_option(
        CreateCommandOption::new(CommandOptionType::User,"user","mention user you want to reset").required(true)
        );
    let user_op = CreateCommandOption::new(CommandOptionType::User, "user","mention the customer").required(true);
    let price_op = CreateCommandOption::new(CommandOptionType::Number, "price","item price (all) not single");
    let count_op = CreateCommandOption::new(CommandOptionType::Integer, "unit","quantity of the item send").required(true);
    let item_op = CreateCommandOption::new(CommandOptionType::String, "item ", "search one of ~17770 item on the list").required(true).set_autocomplete(true);
    let item = AppReg::admin_slash("market", "send item trough distribution then deduct their bounty coin").add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "item","send with item category")
        .add_sub_option(user_op.to_owned()).add_sub_option(item_op.to_owned()).add_sub_option(count_op.to_owned()).add_sub_option(price_op.to_owned())
        ).add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "melee","send with melee weapon category")
        .add_sub_option(user_op.to_owned()).add_sub_option(item_op.to_owned()).add_sub_option(count_op.to_owned()).add_sub_option(price_op.to_owned())
        ).add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "ranged","send with ranged weapon category")
        .add_sub_option(user_op.to_owned()).add_sub_option(item_op.to_owned()).add_sub_option(count_op.to_owned()).add_sub_option(price_op.to_owned())
        ).add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "head","send with head armor category")
        .add_sub_option(user_op.to_owned()).add_sub_option(item_op.to_owned()).add_sub_option(count_op.to_owned()).add_sub_option(price_op.to_owned())
        ).add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "arms","send with arms armor category")
        .add_sub_option(user_op.to_owned()).add_sub_option(item_op.to_owned()).add_sub_option(count_op.to_owned()).add_sub_option(price_op.to_owned())
        ).add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "chest","send with chest armor category")
        .add_sub_option(user_op.to_owned()).add_sub_option(item_op.to_owned()).add_sub_option(count_op.to_owned()).add_sub_option(price_op.to_owned())
        ).add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "waist","send with waist armor category")
        .add_sub_option(user_op.to_owned()).add_sub_option(item_op.to_owned()).add_sub_option(count_op.to_owned()).add_sub_option(price_op.to_owned())
        ).add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "leg","send with leg armor category")
        .add_sub_option(user_op.to_owned()).add_sub_option(item_op.to_owned()).add_sub_option(count_op.to_owned()).add_sub_option(price_op.to_owned())
        );
    vec![
        AppReg::admin_slash("interface", "mhfz user interface"),
        save,
        item
    ]
}
