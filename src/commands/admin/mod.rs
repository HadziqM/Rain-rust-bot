use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::all::CommandOptionType;
use crate::{AppReg,Init};


pub mod interface;
pub mod market;
pub mod purge;
pub mod config;
pub mod query;


pub fn reg(init:&Init)->Vec<CreateCommand>{
    let save = AppReg::admin_slash("reset_save_cd", "reset someone save cooldown").add_option(
        CreateCommandOption::new(CommandOptionType::User,"user","mention user you want to reset").required(true)
        );
    let user_op = CreateCommandOption::new(CommandOptionType::User, "user","mention the customer").required(true);
    let att_op = CreateCommandOption::new(CommandOptionType::Attachment, "json","send your matching .json file").required(true);
    let price_op = CreateCommandOption::new(CommandOptionType::Integer, "price","item price (all) not single");
    let count_op = CreateCommandOption::new(CommandOptionType::Integer, "unit","quantity of the item send max 65025").required(true);
    let item_op = CreateCommandOption::new(CommandOptionType::String, "item ", "search one of ~17770 item on the list").required(true).set_autocomplete(true);
    let item = AppReg::admin_slash("market", "send item trough distribution then deduct their bounty coin").add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "item","send with item category")
        .add_sub_option(user_op.to_owned()).add_sub_option(item_op.to_owned()).add_sub_option(count_op.to_owned()).add_sub_option(price_op.to_owned())
        ).add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "melee","send with melee weapon category")
        .add_sub_option(user_op.to_owned()).add_sub_option(item_op.to_owned()).add_sub_option(price_op.to_owned())
        ).add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "ranged","send with ranged weapon category")
        .add_sub_option(user_op.to_owned()).add_sub_option(item_op.to_owned()).add_sub_option(price_op.to_owned())
        ).add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "head","send with head armor category")
        .add_sub_option(user_op.to_owned()).add_sub_option(item_op.to_owned()).add_sub_option(price_op.to_owned())
        ).add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "arms","send with arms armor category")
        .add_sub_option(user_op.to_owned()).add_sub_option(item_op.to_owned()).add_sub_option(price_op.to_owned())
        ).add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "chest","send with chest armor category")
        .add_sub_option(user_op.to_owned()).add_sub_option(item_op.to_owned()).add_sub_option(price_op.to_owned())
        ).add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "waist","send with waist armor category")
        .add_sub_option(user_op.to_owned()).add_sub_option(item_op.to_owned()).add_sub_option(price_op.to_owned())
        ).add_option(
        CreateCommandOption::new(CommandOptionType::SubCommand, "leg","send with leg armor category")
        .add_sub_option(user_op.to_owned()).add_sub_option(item_op.to_owned()).add_sub_option(price_op.to_owned())
        );
    let mut config = AppReg::admin_slash("config", "change configuration of some bot feature");
    if init.bot_config.gacha{
        config = config.add_option(CreateCommandOption::new(CommandOptionType::SubCommand, "gacha", "send your gacha.json file").add_sub_option(att_op.to_owned()));
    }
    if init.bot_config.server_market{
        config = config.add_option(CreateCommandOption::new(CommandOptionType::SubCommand, "market", "send your market.json file").add_sub_option(att_op.to_owned()));
    }
    vec![
        AppReg::admin_slash("interface", "mhfz user interface"),
        AppReg::admin_slash("purge", "purge user binded and register data on database").add_option(
        CreateCommandOption::new(CommandOptionType::User,"user","mention user you want to purge").required(true)
        ),
        save,
        item,
        config
    ]
}
