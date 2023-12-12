#![allow(unused)]

use std::{collections::HashMap, path::Path, sync::Arc};
use crate::postgres::custom;
use super::*;

#[derive(Serialize,Deserialize,Clone,Default)]
pub struct BBQConfig {
    pub description:String,
    pub cooldown_timer:u32,
    pub public_cooldown :Option<u32>,
    pub icon:String,
    pub thumbnail:String,
    pub rules:Vec<String>,
    pub solo:BountyReward,
    pub multi:BountyReward
}



impl Category {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Gold => "gold",
            Self::Free => "free",
            Self::Event => "event",
            Self::Bronze => "bronze",
            Self::Silver => "silver",
            Self::Custom => "Custom",
        }
    }
    pub fn config_name(&self) -> String {
        format!("bounty_{}.json",self.name())
    }
    pub async fn load(&self) -> Result<HashMap<BBQ,BBQConfig>,BountyErr> {
        let path = Path::new(".").join("static").join(&self.config_name());
        Ok(serde_json::from_slice(&tokio::fs::read(path).await?)?)
    }
}

impl title::Title {
    pub async fn load(&self) -> Result<Self,BountyErr> {
        let path = Path::new(".").join("static").join("title.json");
        Ok(serde_json::from_slice(&tokio::fs::read(path).await?)?)
    }
}

struct Cooldown(HashMap<BBQ,u32>);

impl From<HashMap<BBQ,BBQConfig>> for Cooldown {
    fn from(value: HashMap<BBQ,BBQConfig>) -> Self {
        Cooldown(value.into_iter()
            .map(|(k,v)|(k,v.public_cooldown.unwrap_or(999)))
            .collect())
    }
}


impl BountyGlobal {
    pub fn create() -> Arc<Self> {
        Arc::new(Self { cooldown: Mutex::new(HashMap::new()), submision: Mutex::new(HashMap::new()) })
    }
    pub async fn refresh(&self) -> Result<(),BountyErr> {
        let free = Category::Free.load().await?;
        let new_cd:Cooldown = free.into();
        *self.cooldown.lock().await = new_cd.0;
        Ok(())
    }
    pub async fn caching(&self) -> Result<(),BountyErr> {
        let data = self.submision.lock().await;
        let path = Path::new(".").join("CACHE");
        tokio::fs::write(path, serde_json::to_string(&*data)?.as_bytes()).await?;
        Ok(())
    }
}


