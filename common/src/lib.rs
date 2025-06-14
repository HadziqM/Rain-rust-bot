use std::fmt::Debug;

use lazy_static::lazy_static;
use log::{error, warn};
use sysdir::Sysdir;
use thiserror::Error;
pub mod database;
pub mod gacha;
pub mod item_code;
pub mod setting;
pub mod utils;

#[derive(Debug, Error)]
pub enum CommonError {
    #[error("{0}")]
    Custom(String),
}

impl From<&str> for CommonError {
    fn from(value: &str) -> Self {
        CommonError::Custom(value.to_string())
    }
}

lazy_static! {
    pub static ref SYSDIR: Sysdir = Sysdir::custom_name("RustDiscordBot");
}

pub trait MyResult<T, E> {
    fn log(self) -> Self;
    fn log_warn(self) -> Self;
    fn unwrap_log(self) -> T;
}

impl<T, E> MyResult<T, E> for Result<T, E>
where
    E: Debug,
{
    fn log(self) -> Self {
        if let Err(e) = &self {
            error!("Getting error: {e:#?}");
        }
        self
    }
    fn log_warn(self) -> Self {
        if let Err(e) = &self {
            warn!("Getting error: {e:#?}");
        }
        self
    }
    fn unwrap_log(self) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                error!("Getting error: {e:#?}");
                panic!();
            }
        }
    }
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
