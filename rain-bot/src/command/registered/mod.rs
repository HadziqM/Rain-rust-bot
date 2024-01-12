use crate::{error::MyErr, setup::AppData};

mod account;
mod register;
mod user_info;

pub fn reg() -> Vec<poise::Command<AppData, MyErr>> {
    let mut out = vec![];
    out.append(&mut user_info::reg());
    out.append(&mut register::reg());
    out
}
