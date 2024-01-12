mod registered;
mod admin;

use crate::{error::MyErr, setup::AppData};

pub fn reg() -> Vec<poise::Command<AppData, MyErr>> {
    let mut out = vec![];
    out.append(&mut registered::reg());
    out.append(&mut admin::reg());
    out
}
