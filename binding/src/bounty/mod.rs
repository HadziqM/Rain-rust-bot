use serde::{Serialize,Deserialize};
use crate::bitwise::ItemCode;


#[derive(PartialEq, Eq,Clone,Hash)]  
pub enum Category{
    Bronze,
    Silver,
    Gold,
    Free,
    Event,
    Custom
}

#[derive(Clone,Serialize,Deserialize)]
#[serde(rename_all="snake_case")]
pub enum Methode{
    Solo,
    Multi
}

#[derive(Clone)]
pub struct Title{
    pub bounty_bronze:bool,
    pub bounty_silver:bool,
    pub bounty_gold:bool,
    pub trade_bronze:bool,
    pub trade_silver:bool,
    pub trade_gold:bool,
}

#[derive(PartialEq, Eq,Clone,Hash,Serialize,Deserialize)]
pub enum BBQ{
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

#[derive(Serialize,Deserialize,PartialEq, Eq,Clone,Debug)]
pub struct BountyReward{
    coin:u32,
    ticket:u32,
    items:Vec<ItemCode>
}

#[derive(Clone)]
pub struct Hunter{
    pub member:String,
    pub title:Title,
    pub event:Event,
}
#[derive(Clone)]
pub struct BountySubmit{
    pub method:Methode,
    pub category:Category,
    pub bbq:BBQ,
    pub hunter:Vec<Hunter>,
    pub url:String,
    pub thumb:String,
    pub time:i64,
    pub reward:BountyReward
}
