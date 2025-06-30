use std::num::ParseIntError;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Item code to work on item HEX
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ItemCode {
    pub key: String,
    pub count: u8,
    pub types: u8,
}

impl ItemCode {
    pub fn reverse_key(&self) -> Result<String, ItemCodeError> {
        if self.key.len() != 4 {
            Err(ItemCodeError::InvalidKey(self.key.clone()))
        } else {
            Ok(format!("{}{}", &self.key[2..4], &self.key[0..2],))
        }
    }
    pub fn new(key: String, count: u8, types: u8) -> ItemCode {
        ItemCode { key, count, types }
    }

    pub fn transform_key(&self) -> Result<i32, ParseIntError> {
        i32::from_str_radix(&self.reverse_key().unwrap(), 16)
    }
}

#[derive(Debug, Error)]
pub enum ItemCodeError {
    #[error("Invalid length: {0}")]
    OddLength(usize),
    #[error("Invalid key: {0}")]
    InvalidKey(String),
    #[error("No Item")]
    NoItem,
    #[error("ParseInt error: {0}")]
    ParseInt(#[from] ParseIntError),
}
