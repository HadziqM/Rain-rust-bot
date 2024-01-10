use crate::{setup::AppData, error::MyErr};

mod user_info;
mod account;
mod register;


pub fn reg() -> Vec<poise::Command<AppData,MyErr>> {
    let mut out = vec![];
    out.append(&mut user_info::reg());
    out.append(&mut register::reg());
    out
}
