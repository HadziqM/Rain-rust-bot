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
    pub async fn check(data:&str)->Result<(),MyErr>{
        let x = serde_json::from_str::<Self>(data)?;
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
