use std::num::ParseIntError;
use std::fmt;
use serde::{Serialize,Deserialize};

pub mod distribution;

#[derive(Debug,Serialize,Deserialize,Clone,PartialEq,Eq)]
pub struct ItemCode {
    pub key: String,
    pub count: u16,
    pub types: u8,
}
impl Default for ItemCode {
    fn default() -> Self {
        ItemCode { key: "0700".to_string(), count: 1, types: 7 }
    }
}

//create own error enum
#[derive(Debug)]
pub enum BitwiseError {
    OddLength,
    InvalidKey,
    NoItem,
    ParseInt(ParseIntError),
    Sqlx(sqlx::Error),
}

impl fmt::Display for BitwiseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BitwiseError::OddLength => "input string has an odd number of bytes".fmt(f),
            BitwiseError::InvalidKey => "the key for converting endian is invalid length".fmt(f),
            BitwiseError::NoItem => "no item on the selected data".fmt(f),
            BitwiseError::ParseInt(e) => e.fmt(f),
            BitwiseError::Sqlx(e)=>e.fmt(f)
        }
    }
}
impl std::error::Error for BitwiseError {}

//implement parser for error
impl From<ParseIntError> for BitwiseError {
    fn from(e: ParseIntError) -> Self {
        BitwiseError::ParseInt(e)
    }
}
impl From<sqlx::Error> for BitwiseError{
    fn from(value: sqlx::Error) -> Self {
        BitwiseError::Sqlx(value)
    }
}

pub struct Bitwise<'a>{
    item:&'a [ItemCode]
}

impl<'a> Bitwise<'a>{
    pub fn decode(hex_value:&str)->Result<Vec<u8>,BitwiseError>{
        if hex_value.len()%2 != 0{
            return Err(BitwiseError::OddLength);
        }
        //pair it two then decode
        (0..hex_value.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex_value[i..i + 2], 16).map_err(|e| e.into()))
            .collect()
    }
}
