use crate::bitwise::ItemCode;
use serde::{Deserialize, Serialize};

use super::{Category, BBQ};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Trigger {
    BountyStage { category: Category, bbq: BBQ },
    BountyCoin { coin: i32 },
    Manual,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Progresion {
    GoldBounty,
    SilverBounty,
    BronzeBounty,
    GoldTrading,
    SilverTrading,
    BronzeTrading,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TitleImage {
    pub url: String,
    pub diameter: u32,
    pub x_start: u32,
    pub y_start: u32,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TitleType {
    Progresion { name: Progresion },
    Free { name: String },
    Reward { name: String, reward: Vec<ItemCode> },
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Title {
    #[serde(rename = "type")]
    title_type: TitleType,
    image: TitleImage,
    trigger: Trigger,
}
