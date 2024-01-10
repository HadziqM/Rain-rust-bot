mod registered;


use crate::{setup::AppData, error::MyErr};

pub fn reg() -> Vec<poise::Command<AppData,MyErr>> {
    let mut out = vec![];
    out.append(&mut registered::reg());
    out
}
