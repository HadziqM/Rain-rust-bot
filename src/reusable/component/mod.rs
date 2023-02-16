pub mod error;
pub mod widget;
pub mod guide;
pub mod card;
pub mod registered;
pub mod discord;
pub mod json;
pub mod new_error;

pub struct Components;

#[derive(Debug)]
pub enum MyErr{
    Serenity(serenity::Error),
    Tokio(tokio::io::Error),
    Utf8(std::str::Utf8Error),
    Serde(serde_json::Error),
    ByteWise(super::bitwise::BitwiseError),
    Custom(String)
}
impl std::error::Error for MyErr {}
impl std::fmt::Display for MyErr{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            MyErr::Tokio(x)=>x.fmt(f),
            MyErr::Serenity(x)=>x.fmt(f),
            MyErr::Utf8(x)=>x.fmt(f),
            MyErr::Serde(x)=>x.fmt(f),
            MyErr::Custom(x)=>x.fmt(f),
            MyErr::ByteWise(x)=>x.fmt(f)
        }
    }
}
impl From<super::bitwise::BitwiseError> for MyErr {
    fn from(value: super::bitwise::BitwiseError) -> Self {
        MyErr::ByteWise(value)
    }
}
impl From<tokio::io::Error> for MyErr{
    fn from(value: tokio::io::Error) -> Self {
        MyErr::Tokio(value)
    }
}
impl From<serenity::Error> for MyErr {
    fn from(value: serenity::Error) -> Self {
        MyErr::Serenity(value)
    }
}
impl From<std::str::Utf8Error> for MyErr{
    fn from(value: std::str::Utf8Error) -> Self {
        MyErr::Utf8(value)
    }
}
impl From<serde_json::Error> for MyErr {
    fn from(value: serde_json::Error) -> Self {
        MyErr::Serde(value)
    }
}
