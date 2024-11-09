use lazy_static::lazy_static;
use sysdir::Sysdir;
pub mod database;
pub mod item_code;
pub mod setting;
pub mod utils;

lazy_static! {
    pub static ref SYSDIR: Sysdir = Sysdir::custom_name("RustDiscordBot");
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
