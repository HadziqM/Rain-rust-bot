use crate::{setup::AppData, error::MyErr};

mod moderate;


pub fn reg() -> Vec<poise::Command<AppData,MyErr>> {
    let mut out = vec![];
    out.append(&mut moderate::reg());
    out
}
