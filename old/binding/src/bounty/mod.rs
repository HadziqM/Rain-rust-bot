use crate::{bitwise::ItemCode, postgres::card::Event};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::Mutex;

use title::Progresion;

pub mod config;
pub mod submit;
pub mod title;

pub struct BountyGlobal {
    pub cooldown: Mutex<HashMap<BBQ, u32>>,
    pub submision: Mutex<HashMap<String, BountySubmit>>,
}

#[derive(Debug)]
pub enum BountyErr {
    Custom(String),
    Tokio(tokio::io::Error),
    Serde(serde_json::Error),
}

impl std::error::Error for BountyErr {}

impl std::fmt::Display for BountyErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Custom(x) => x.fmt(f),
            Self::Tokio(x) => x.fmt(f),
            Self::Serde(x) => x.fmt(f),
        }
    }
}

impl From<tokio::io::Error> for BountyErr {
    fn from(value: tokio::io::Error) -> Self {
        Self::Tokio(value)
    }
}

impl From<serde_json::Error> for BountyErr {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}
impl From<&str> for BountyErr {
    fn from(value: &str) -> Self {
        Self::Custom(value.to_string())
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Bronze,
    Silver,
    Gold,
    Free,
    Event,
    Custom,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Methode {
    Solo,
    Multi,
}

#[derive(PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum BBQ {
    BBQ01,
    BBQ02,
    BBQ03,
    BBQ04,
    BBQ05,
    BBQ06,
    BBQ07,
    BBQ08,
    BBQ09,
    BBQ10,
    BBQ11,
    BBQ12,
    BBQ13,
    BBQ14,
    BBQ15,
    BBQ16,
    BBQ17,
    BBQ18,
    BBQ19,
    BBQ20,
    BBQ21,
    BBQ22,
    BBQ23,
    BBQ24,
    BBQ25,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Default)]
pub struct BountyReward {
    coin: u32,
    ticket: u32,
    items: Vec<ItemCode>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Hunter {
    pub member: String,
    pub title: Progresion,
    pub event: Event,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct BountySubmit {
    pub method: Methode,
    pub category: Category,
    pub bbq: BBQ,
    pub hunter: Vec<Hunter>,
    pub url: String,
    pub thumb: String,
    pub time: i64,
    pub reward: BountyReward,
}
