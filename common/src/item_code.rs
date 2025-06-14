use std::num::ParseIntError;

use macros::Wrapper;
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
        i32::from_str_radix(&self.key, 16)
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

#[derive(Wrapper)]
pub struct ItemHexOperation(Vec<ItemCode>);

impl ItemHexOperation {
    pub fn new(data: Vec<ItemCode>) -> ItemHexOperation {
        Self(data)
    }
    // pub fn encode(&self) -> Result<Vec<u8>, ItemCodeError> {
    //     if self.is_empty() {
    //         return Err(ItemCodeError::NoItem);
    //     }
    //     let mut hex_value = format!("{:04X}", self.len());
    //     for code in self.iter() {
    //         hex_value.push_str(&format!(
    //             "{:02X}0000{}0000{:04X}00000000",
    //             code.types,
    //             code.reverse_key()?,
    //             code.count
    //         ));
    //     }
    //     if hex_value.len() % 2 != 0 {
    //         return Err(ItemCodeError::OddLength(hex_value.len()));
    //     }
    //     //pair it two then decode
    //     (0..hex_value.len())
    //         .step_by(2)
    //         .map(|i| u8::from_str_radix(&hex_value[i..i + 2], 16).map_err(|e| e.into()))
    //         .collect()
    // }
}
