pub mod submit;

use crate::AppReg;
use serenity::all::*;

pub fn reg() -> Vec<CreateCommand> {
    let cooldown = AppReg::normal_slash("cooldown", "cooldown")
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
                    AppReg::int_option("bounty", "the bounty needed to reset").required(true),
                )
                .add_sub_option(
                    AppReg::int_option("cooldown", "set to match input").required(true),
                ),
        );
    vec![cooldown, AppReg::message_context("🎮 Submit")]
}
