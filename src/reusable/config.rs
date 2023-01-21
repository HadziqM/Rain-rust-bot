use std::fs::read_to_string;
use serde::Deserialize;
use std::error::Error;


#[derive(Debug,Deserialize,Clone)]
pub struct Init {
    pub(crate) token: String,
    pub(crate) prefix: String,
    pub(crate) err_channel: String
}

pub fn get_config()->Result<Init,Box<dyn Error>>{
    let input = read_to_string("./config.json")?;
    Ok(serde_json::from_str(&input)?)
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn config() {
        let idk = get_config().unwrap();
        assert_eq!(idk.prefix,"%".to_string());
    }
}
