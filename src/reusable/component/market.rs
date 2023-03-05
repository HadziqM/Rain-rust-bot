use std::path::{PathBuf,Path};
use std::num::NonZeroU64;
use std::collections::HashMap;
use serde::{Serialize,Deserialize};
use serde_json::Value;
use serenity::all::*;
use super::super::bitwise::ItemCode;
use super::MyErr;
use crate::{ItemPedia,Init};

#[derive(Serialize,Deserialize,Clone)]
pub struct Market{
    pub market:Vec<Item>
}
#[derive(Serialize,Deserialize,Clone)]
pub struct Item{
    pub item:ItemCode,
    pub treshold:u32,
    pub price:u32
}
impl Default for Market {
    fn default() -> Self {
        Market { market: Vec::new() }
    }
}
impl Default for Item {
    fn default() -> Self {
        Item { item: ItemCode::default(), treshold: 0, price: 99999999 }
    }
}
impl Market {
    fn path()->PathBuf{
        Path::new(".").join("static").join("market.json")
    }
    pub async fn new()->Result<Market,MyErr>{
        Ok(serde_json::from_str(&tokio::fs::read_to_string(&Market::path()).await?)?)
    }
    pub fn auto(&self,item:&ItemPedia,focus:&str)->Vec<AutocompleteChoice>{
        self.market.iter().filter_map(|y|{
            let name = format!("{} - {}",&y.item.key,y.item.text(item).unwrap());
            let foc = focus.to_lowercase();
            if name.to_lowercase().contains(&foc) || name.to_lowercase().starts_with(&foc){
                if y.treshold != 0{
                    return Some(AutocompleteChoice{value:Value::String(y.item.key.to_owned())
                        ,name});
                }
            }
            None
        }).collect()
    }
    pub async fn save(&self)->Result<(),MyErr>{
        Ok(tokio::fs::write(&Market::path(),
            serde_json::to_string_pretty(&self)?.as_bytes()).await?)
    }
    pub async fn check(data:&[u8])->Result<(),MyErr>{
        let x = serde_json::from_slice::<Self>(data)?;
        x.save().await?;
        Ok(())
    }
    pub fn item_hash(&self)->HashMap<String,Item>{
        self.market.iter().map(|y|(y.item.key.to_owned(),y.clone())).collect()
    }
    pub fn bought(&mut self,bought:&Item){
        for item in self.market.iter_mut(){
            if item.item == bought.item{
                *item = bought.to_owned()
            }
        }
    }
    pub fn make_embed(&self,pedia:&ItemPedia)->CreateEmbed{
        use crate::reusable::utils::Color;
        let tit = format!("**ITEMS**\t\t\t\t**STOCK**\n");
        let desc:String = self.market.iter().map(|x|format!("\n{}\t\t{}\tavailable",x.item.text(pedia).unwrap(),x.treshold)).collect();
        CreateEmbed::new().color(Color::Random.throw()).title("Server Market Stall").description(["```\n",&tit,&desc,"\n```"].concat())
    }
    pub async fn send(&self,ctx:&Context,init:&mut Init,msg:&Message,pedia:&ItemPedia)->Result<(),MyErr>{
        if init.server_role.admin_role == msg.author.id.get(){
            let mid = msg.channel_id.send_message(&ctx.http, CreateMessage::new().embed(self.make_embed(pedia))).await?;
            init.log_channel.market_channel = msg.channel_id.get();
            init.log_channel.market_channel_msg = mid.id.get();
            init.bot_config.server_market = true;
            return Ok(());
        }
        Err(MyErr::Custom("You Cant youse admin only Command".to_string()))
    }
    pub async fn update(&self,ctx:&Context,init:&Init,pedia:&ItemPedia)->Result<(),MyErr>{
        let mut msg = ChannelId(NonZeroU64::new(init.log_channel.market_channel).unwrap())
            .message(&ctx.http, init.log_channel.market_channel_msg).await?;
        msg.edit(&ctx.http, EditMessage::new().embed(self.make_embed(pedia))).await?;
        Ok(())
    }
    pub async fn update_new(ctx:&Context,init:&Init,pedia:&ItemPedia)->Result<(),MyErr>{
        let x = Market::new().await?;
        let mut msg = ChannelId(NonZeroU64::new(init.log_channel.market_channel).unwrap())
            .message(&ctx.http, init.log_channel.market_channel_msg).await?;
        msg.edit(&ctx.http, EditMessage::new().embed(x.make_embed(pedia))).await?;
        Ok(())
    }
    pub fn currency(cur:i64)->String{
        let inp = cur.to_string(); 
        let x = inp.chars().rev();
        let mut y =Vec::new();
        for (i,c) in x.enumerate(){
            if i%3 == 0{
                y.push('.')
            }
            y.push(c)
        }
        let z:String = y[1..].iter().rev().collect();
        ["Bc ",&z,",00"].concat()
    }

}
#[derive(Serialize,Deserialize,PartialEq, Eq)]
pub struct MealMenu{
    pub id:i32,
    pub level:i32,
    pub name:String
}
#[derive(Serialize,Deserialize,PartialEq, Eq)]
pub struct Meal{
    pub meals:Vec<MealMenu>
}
impl Meal{
    fn path()->PathBuf{
        Path::new(".").join("static").join("meals.json")
    }
    pub async fn new()->Result<Self,MyErr>{
        let file = tokio::fs::read_to_string(Meal::path()).await?;
        Ok(serde_json::from_str(&file)?)
    }
    pub async fn save(&self)->Result<(),MyErr>{
        let file = serde_json::to_string_pretty(&self)?;
        Ok(tokio::fs::write(Meal::path(), file.as_bytes()).await?)
    }
    pub async fn check(data:&[u8])->Result<(),MyErr>{
        let x = serde_json::from_slice::<Self>(data)?;
        x.save().await?;
        Ok(())
    }
}
impl Default for MealMenu {
    fn default() -> Self {
        MealMenu { id: 369, level: 3, name: "idk really".to_string() }
    }
}
impl Default for Meal {
    fn default() -> Self {
        Meal { meals: vec![MealMenu::default(),MealMenu::default(),MealMenu::default()] }
    }
}

#[derive(Serialize,Deserialize,Clone)]
pub struct Trading{
    pub market:TradingMenu,
    pub bar:TradingMenu,
    pub casino:TradingMenu,
    pub jewelry:TradingMenu,
    pub restourant:TradingMenu,
    pub guild:TradingMenu
}

#[derive(Serialize,Deserialize,Clone)]
pub struct TradingMenu{
    pub enabled:bool,
    pub price:i32
}
impl Trading{
    fn path()->PathBuf{
        Path::new(".").join("static").join("trading.json")
    }
    pub async fn new()->Result<Self,MyErr>{
        let file = tokio::fs::read_to_string(Trading::path()).await?;
        Ok(serde_json::from_str(&file)?)
    }
    pub async fn save(&self)->Result<(),MyErr>{
        let file = serde_json::to_string_pretty(&self)?;
        Ok(tokio::fs::write(Trading::path(), file.as_bytes()).await?)
    }
    pub async fn check(data:&[u8])->Result<(),MyErr>{
        let x = serde_json::from_slice::<Self>(data)?;
        x.save().await?;
        Ok(())
    }
}
impl Default for TradingMenu{
    fn default() -> Self {
        TradingMenu { enabled: false, price: 100 }
    }
}
impl Default for Trading {
    fn default() -> Self {
        Trading { market: TradingMenu::default(),
        bar: TradingMenu::default(),
        casino: TradingMenu::default(),
        jewelry: TradingMenu::default(),
        restourant: TradingMenu::default(),
        guild:TradingMenu::default()
        }
    }
}
#[derive(Serialize,Deserialize,PartialEq, Eq)]
pub struct TagItem{
    pub desc:String,
    pub command:String,
    pub url:String
}
#[derive(Serialize,Deserialize,PartialEq, Eq)]
pub struct Tag{
    pub tag:Vec<TagItem>
}
impl Tag{
    fn path()->PathBuf{
        Path::new(".").join("static").join("tag.json")
    }
    pub async fn new()->Result<Self,MyErr>{
        let file = tokio::fs::read_to_string(Tag::path()).await?;
        Ok(serde_json::from_str(&file)?)
    }
    pub async fn save(&self)->Result<(),MyErr>{
        let file = serde_json::to_string_pretty(&self)?;
        Ok(tokio::fs::write(Tag::path(), file.as_bytes()).await?)
    }
    pub async fn check(data:&[u8])->Result<(),MyErr>{
        let x = serde_json::from_slice::<Self>(data)?;
        x.save().await?;
        Ok(())
    }
}
impl Default for TagItem {
    fn default() -> Self {
        TagItem { desc: "".to_owned(), command: "".to_owned(), url: "".to_owned() }
    }
}
impl Default for Tag {
    fn default() -> Self {
        Tag { tag: vec![TagItem::default(),TagItem::default()] }
    }
}

#[cfg(test)]
mod testing{
    use super::*;
    #[tokio::test]
    #[ignore = "already have"]
    async fn default() {
        let x = Tag::default();
        x.save().await.unwrap();
    }
}
