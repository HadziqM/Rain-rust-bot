pub mod submit;
pub mod pedia;
pub mod cooldown;

use crate::AppReg;
use serenity::all::*;
use crate::reusable::component::bounty::{BBQ,Methode,Category};

pub fn reg() -> Vec<CreateCommand> {
    let mut methode = AppReg::str_option("methode", "methode used to clear bounty").required(true);
    let mut category = AppReg::str_option("category", "category you pick for bounty").required(true);
    let mut bbq = AppReg::str_option("bbq", "stage you pick for bounty").required(true);
    for i in Methode::option_str(){
        methode = methode.add_string_choice(i.1, i.0);
    }
    for i in Category::option_str(){
        category = category.add_string_choice(i.1, i.0);
    }
    for i in BBQ::option_str(){
        bbq = bbq.add_string_choice(i.1, i.0);
    }

    let cooldown = AppReg::admin_slash("cooldown", "cooldown")
        .add_option(AppReg::subcommand(
            "refresh",
            "refresh free category bounty cd",
        ))
        .add_option(
            AppReg::subcommand("user", "refresh user bounty cd")
                .add_sub_option(AppReg::user_option("user", "mention the user").required(true)),
        )
        .add_option(
            AppReg::subcommand("bounty", "set specified bounty cd")
                .add_sub_option(
                    bbq.to_owned(),
                )
                .add_sub_option(
                    AppReg::int_option("cooldown", "set to match input").required(true),
                ),
        );
    let dist =AppReg::admin_slash("distribution", "distribute bounty reward to player")
        .add_option(methode.to_owned())
        .add_option(category.to_owned())
        .add_option(bbq.to_owned())
        .add_option(AppReg::str_option("mentions", "mention all user (can be more than 1) to send"));
    let bounty = AppReg::normal_slash("bounty", "command group for bounty")
        .add_option(AppReg::subcommand("submit", "submit your bounty useing second methode")
            .add_sub_option(AppReg::att_option("image", "the image proof for your bounty").required(true))
            .add_sub_option(methode.clone())
            .add_sub_option(category.clone())
            .add_sub_option(bbq.clone())
            .add_sub_option(AppReg::str_option("mentions", "your team on bounty if multiplier")))
        .add_option(AppReg::subcommand("pedia", "your bounty wikipedia")
            .add_sub_option(category.clone())
            .add_sub_option(bbq.clone()));
    vec![AppReg::message_context("🎮 Submit")
        ,bounty
        ,dist
        ,cooldown
    ]
}
