use std::num::ParseIntError;
use std::fmt;

pub mod distribution;

//create own error enum
#[derive(Debug)]
pub enum BitwiseError {
    OddLength,
    InvalidKey,
    ParseInt(ParseIntError),
    Sqlx(sqlx::Error),
}

impl fmt::Display for BitwiseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BitwiseError::OddLength => "input string has an odd number of bytes".fmt(f),
            BitwiseError::InvalidKey => "the key for converting endian is invalid length".fmt(f),
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

pub struct Bitwise;

impl Bitwise{
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
