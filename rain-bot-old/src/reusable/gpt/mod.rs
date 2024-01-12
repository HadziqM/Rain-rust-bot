pub mod chat;
pub mod image;

use crate::{Init, MyErr};
use reqwest::header::HeaderMap;

pub struct Gpt {
    head: HeaderMap,
}

impl From<reqwest::header::InvalidHeaderValue> for MyErr {
    fn from(value: reqwest::header::InvalidHeaderValue) -> Self {
        MyErr::Custom(format!("reqwest error on invalid header value :{value:?}"))
    }
}
impl From<reqwest::Error> for MyErr {
    fn from(value: reqwest::Error) -> Self {
        MyErr::Image(value.into())
    }
}
impl Gpt {
    pub fn new(init: &Init) -> Result<Self, MyErr> {
        let mut head = HeaderMap::new();
        head.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        head.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&format!("Bearer {}", &init.chat_gpt.token))?,
        );
        Ok(Gpt { head })
    }
}
