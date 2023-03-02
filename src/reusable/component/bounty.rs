use std::path::Path;
use std::path::PathBuf;

use serenity::all::*;
use crate::reusable::utils::Color;
use super::MyErr;


pub struct Title{
    pub bounty_bronze:bool,
    pub bounty_silver:bool,
    pub bounty_gold:bool,
    pub trade_bronze:bool,
    pub trade_silver:bool,
    pub trade_gold:bool,
}
impl Title{
    fn matching(code:u8,math:u8)->bool{
        if code & math == math{
            return true;
        }
        false
    }
    pub fn new(code:u8)->Self{
        Title { bounty_bronze: Title::matching(code, 1), 
            bounty_silver: Title::matching(code,2), 
            bounty_gold: Title::matching(code,4), 
            trade_bronze: Title::matching(code, 8), 
            trade_silver: Title::matching(code, 16),
            trade_gold: Title::matching(code, 32) }
    }
    pub fn bonus(&self)->f32{
        let mut out = 1.0;
        if self.bounty_gold{
            out += 0.3
        }if self.bounty_silver{
            out += 0.2
        }if self.bounty_bronze{
            out += 0.1
        }
        out
    }
    pub fn discount(&self)->f32{
        let mut out = 1.0;
        if self.trade_gold{
            out -= 0.3
        }if self.trade_silver{
            out -= 0.2
        }if self.trade_bronze{
            out -= 0.1
        }
        out
    }
}


pub enum Methode{
    Solo,
    Multi
}
impl Methode{
    pub fn new(code:u8)->Self{
        match code {
            0 => Methode::Solo,
            _ => Methode::Multi,
        }
    }
    pub fn name(&self)->String{
        match self{
            Methode::Solo=>"Solo".to_owned(),
            Methode::Multi=>"Multiplayer".to_owned(),
        }
    }
    pub fn option_str()->Vec<(String,String)>{
        vec![
            ("0".to_owned(),"Solo".to_owned()),
            ("1".to_owned(),"Multi".to_owned()),
        ]
    }
}


    
pub enum Category{
    Bronze,
    Silver,
    Gold,
    Free,
    Event
}
impl Category{
    pub fn new(code:u8)->Result<Category,MyErr>{
        match code{
            0=>Ok(Category::Bronze),
            1=>Ok(Category::Silver),
            2=>Ok(Category::Gold),
            3=>Ok(Category::Free),
            4=>Ok(Category::Event),
            _=>Err(MyErr::Custom("cant get category".to_string()))
        }
    }
    pub fn name(&self)->String{
        match self{
            Category::Event=>"Event".to_string(),
            Category::Free=>"Free".to_owned(),
            Category::Gold=>"Gold".to_owned(),
            Category::Silver=>"Silver".to_owned(),
            Category::Bronze=>"Bronze".to_owned()
        }
    }
    pub fn color(&self)->Color{
        match self{
            Self::Gold=>Color::Gold,
            Self::Silver=>Color::Silver,
            Self::Bronze=>Color::Bronze,
            Self::Free=>Color::Green,
            Self::Event=>Color::Blue
        }
    }
    pub fn option_str()->Vec<(String,String)>{
        vec![
            ("0".to_owned(),"Bronze".to_owned()),
            ("1".to_owned(),"Silver".to_owned()),
            ("2".to_owned(),"Gold".to_owned()),
            ("3".to_owned(),"Free".to_owned()),
            ("4".to_owned(),"Event".to_owned()),
        ]
    }
    fn get_bounty<'a>(&self,bounty:&'a Bounty)->&'a BountyBBQ{
        match self{
            Self::Gold=>&bounty.gold,
            Self::Silver=>&bounty.silver,
            Self::Bronze=>&bounty.bronze,
            Self::Free=>&bounty.free,
            Self::Event=>&bounty.event
        }
    }

}



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
impl BBQ{
    pub fn new(code:u8)->Result<BBQ,MyErr>{
        match code{
            0=>Ok(BBQ::BBQ01),
            1=>Ok(BBQ::BBQ02),
            2=>Ok(BBQ::BBQ03),
            3=>Ok(BBQ::BBQ04),
            4=>Ok(BBQ::BBQ05),
            5=>Ok(BBQ::BBQ06),
            6=>Ok(BBQ::BBQ07),
            7=>Ok(BBQ::BBQ08),
            8=>Ok(BBQ::BBQ09),
            9=>Ok(BBQ::BBQ10),
            10=>Ok(BBQ::BBQ11),
            11=>Ok(BBQ::BBQ12),
            12=>Ok(BBQ::BBQ13),
            13=>Ok(BBQ::BBQ14),
            14=>Ok(BBQ::BBQ15),
            15=>Ok(BBQ::BBQ16),
            16=>Ok(BBQ::BBQ17),
            17=>Ok(BBQ::BBQ18),
            18=>Ok(BBQ::BBQ19),
            19=>Ok(BBQ::BBQ20),
            20=>Ok(BBQ::BBQ21),
            21=>Ok(BBQ::BBQ22),
            22=>Ok(BBQ::BBQ23),
            23=>Ok(BBQ::BBQ24),
            24=>Ok(BBQ::BBQ25),
            _=>Err(MyErr::Custom("cant get bbq".to_string()))
        }
    }
    pub fn name(&self)->String{
        match self{
            BBQ::BBQ01=>"BBQ01".to_owned(),
            BBQ::BBQ02=>"BBQ02".to_owned(),
            BBQ::BBQ03=>"BBQ03".to_owned(),
            BBQ::BBQ04=>"BBQ04".to_owned(),
            BBQ::BBQ05=>"BBQ05".to_owned(),
            BBQ::BBQ06=>"BBQ06".to_owned(),
            BBQ::BBQ07=>"BBQ07".to_owned(),
            BBQ::BBQ08=>"BBQ08".to_owned(),
            BBQ::BBQ09=>"BBQ09".to_owned(),
            BBQ::BBQ10=>"BBQ10".to_owned(),
            BBQ::BBQ11=>"BBQ11".to_owned(),
            BBQ::BBQ12=>"BBQ12".to_owned(),
            BBQ::BBQ13=>"BBQ13".to_owned(),
            BBQ::BBQ14=>"BBQ14".to_owned(),
            BBQ::BBQ15=>"BBQ15".to_owned(),
            BBQ::BBQ16=>"BBQ16".to_owned(),
            BBQ::BBQ17=>"BBQ17".to_owned(),
            BBQ::BBQ18=>"BBQ18".to_owned(),
            BBQ::BBQ19=>"BBQ19".to_owned(),
            BBQ::BBQ20=>"BBQ20".to_owned(),
            BBQ::BBQ21=>"BBQ21".to_owned(),
            BBQ::BBQ22=>"BBQ22".to_owned(),
            BBQ::BBQ23=>"BBQ23".to_owned(),
            BBQ::BBQ24=>"BBQ24".to_owned(),
            BBQ::BBQ25=>"BBQ25".to_owned(),
        }
    }
    pub fn option_str()->Vec<(String,String)>{
        vec![
            ("0".to_owned(),"BBQ01".to_owned()),
            ("1".to_owned(),"BBQ02".to_owned()),
            ("2".to_owned(),"BBQ03".to_owned()),
            ("3".to_owned(),"BBQ04".to_owned()),
            ("4".to_owned(),"BBQ05".to_owned()),
            ("5".to_owned(),"BBQ06".to_owned()),
            ("6".to_owned(),"BBQ07".to_owned()),
            ("7".to_owned(),"BBQ08".to_owned()),
            ("8".to_owned(),"BBQ09".to_owned()),
            ("9".to_owned(),"BBQ10".to_owned()),
            ("10".to_owned(),"BBQ11".to_owned()),
            ("11".to_owned(),"BBQ12".to_owned()),
            ("12".to_owned(),"BBQ13".to_owned()),
            ("13".to_owned(),"BBQ14".to_owned()),
            ("14".to_owned(),"BBQ15".to_owned()),
            ("15".to_owned(),"BBQ16".to_owned()),
            ("16".to_owned(),"BBQ17".to_owned()),
            ("17".to_owned(),"BBQ18".to_owned()),
            ("18".to_owned(),"BBQ19".to_owned()),
            ("19".to_owned(),"BBQ20".to_owned()),
            ("20".to_owned(),"BBQ21".to_owned()),
            ("21".to_owned(),"BBQ22".to_owned()),
            ("22".to_owned(),"BBQ23".to_owned()),
            ("23".to_owned(),"BBQ24".to_owned()),
            ("24".to_owned(),"BBQ25".to_owned()),
        ]
    }
    fn get_bounty<'a>(&self,bbq:&'a BountyBBQ)->&'a BountyDesc{
        match self{
            BBQ::BBQ01=>&bbq.bbq1,
            BBQ::BBQ02=>&bbq.bbq2,
            BBQ::BBQ03=>&bbq.bbq3,
            BBQ::BBQ04=>&bbq.bbq4,
            BBQ::BBQ05=>&bbq.bbq5,
            BBQ::BBQ06=>&bbq.bbq6,
            BBQ::BBQ07=>&bbq.bbq7,
            BBQ::BBQ08=>&bbq.bbq8,
            BBQ::BBQ09=>&bbq.bbq9,
            BBQ::BBQ10=>&bbq.bbq10,
            BBQ::BBQ11=>&bbq.bbq11,
            BBQ::BBQ12=>&bbq.bbq12,
            BBQ::BBQ13=>&bbq.bbq13,
            BBQ::BBQ14=>&bbq.bbq14,
            BBQ::BBQ15=>&bbq.bbq15,
            BBQ::BBQ16=>&bbq.bbq16,
            BBQ::BBQ17=>&bbq.bbq17,
            BBQ::BBQ18=>&bbq.bbq18,
            BBQ::BBQ19=>&bbq.bbq19,
            BBQ::BBQ20=>&bbq.bbq20,
            BBQ::BBQ21=>&bbq.bbq21,
            BBQ::BBQ22=>&bbq.bbq22,
            BBQ::BBQ23=>&bbq.bbq23,
            BBQ::BBQ24=>&bbq.bbq24,
            BBQ::BBQ25=>&bbq.bbq25,
        }
    }
}

pub struct Hunter{
    pub member:User,
    pub title:Title,
    pub name:String
}
pub struct BountySubmit{
    pub method:Methode,
    pub category:Category,
    pub bbq:BBQ,
    pub hunter:Vec<Hunter>,
    pub url:String,
    pub thumb:String
}
use super::Components;
use crate::PgConn;
impl Hunter {
    async fn new(user:User,msg:&Message,pg:&mut PgConn<'_>)->Result<Vec<Hunter>,MyErr>{
        let mut vec = vec![user];
        let mut hunter = Vec::new();
        for i in &msg.mentions{
            vec.push(i.to_owned())
        }
        for us in vec{
            pg.did = us.id.to_string();
            let event = pg.get_event().await
                .ok().ok_or(MyErr::Custom(format!("{} most likely isnt binded yet please check",us.to_string())))?;
            let title = Title::new(u8::try_from(event.title).unwrap());
            hunter.push(Hunter{member:us.to_owned(),title,name:event.name.to_owned()});
        }
        Ok(hunter)
    }
}

impl BountySubmit{
    async fn new(user:User,msg:&Message,pg:&mut PgConn<'_>,bounty:&Bounty,url:&str,method:Methode,bbq:BBQ,category:Category)
        ->Result<BountySubmit,MyErr>{
        let hunter = Hunter::new(user, msg, pg).await?;
        let idk = category.get_bounty(bounty);
        let desc = bbq.get_bounty(idk);
        Ok(BountySubmit{hunter,method,bbq,category,url:url.to_owned(),thumb:desc.icon.to_owned()})
    }
    pub fn embed(&self)->CreateEmbed{
        let title = format!("{} {} {}",self.category.name(),self.bbq.name(),self.method.name());
        let mut desc = Vec::new();
        for i in &self.hunter{
            desc.push("\n".to_owned());
            desc.push(format!("```\nname\t:\t{}\ndiscord\t:\t{}\n```",&i.name,&i.member.name));
        }
        let res = desc[1..].concat();
        let author = &self.hunter[0].member;
        CreateEmbed::new().title(title).description(res).thumbnail(&self.thumb).image(&self.url)
            .author(CreateEmbedAuthor::new(&author.name).url(author.face())).color(self.category.color().throw())
    }
    pub fn button(&self)->Vec<CreateActionRow>{
        let accept = Components::normal_button("Approve", "bounty_a", ButtonStyle::Primary, "👌");
        let reject = Components::normal_button("Reject", "bounty_r", ButtonStyle::Danger, "👎");
        let arow = CreateActionRow::Buttons(vec![accept,reject]);
        vec![arow]
    }
}

use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,PartialEq, Eq)]
pub struct Bounty{
    pub gold:BountyBBQ,
    pub silver:BountyBBQ,
    pub bronze:BountyBBQ,
    pub free:BountyBBQ,
    pub event:BountyBBQ
}
#[derive(Serialize,Deserialize,PartialEq, Eq)]
pub struct BountyBBQ{
    pub bbq1:BountyDesc,
    pub bbq2:BountyDesc,
    pub bbq3:BountyDesc,
    pub bbq4:BountyDesc,
    pub bbq5:BountyDesc,
    pub bbq6:BountyDesc,
    pub bbq7:BountyDesc,
    pub bbq8:BountyDesc,
    pub bbq9:BountyDesc,
    pub bbq10:BountyDesc,
    pub bbq11:BountyDesc,
    pub bbq12:BountyDesc,
    pub bbq13:BountyDesc,
    pub bbq14:BountyDesc,
    pub bbq15:BountyDesc,
    pub bbq16:BountyDesc,
    pub bbq17:BountyDesc,
    pub bbq18:BountyDesc,
    pub bbq19:BountyDesc,
    pub bbq20:BountyDesc,
    pub bbq21:BountyDesc,
    pub bbq22:BountyDesc,
    pub bbq23:BountyDesc,
    pub bbq24:BountyDesc,
    pub bbq25:BountyDesc,
}
use super::super::bitwise::ItemCode;
#[derive(Serialize,Deserialize,PartialEq, Eq)]
pub struct BountyDesc {
    pub description:String,
    pub cooldown:u32,
    pub icon:String,
    pub thumbnail:String,
    pub rules:Vec<String>,
    pub solo:BountyReward,
    pub multi:BountyReward
}
#[derive(Serialize,Deserialize,PartialEq, Eq)]
pub struct BountyReward{
    coin:u32,
    ticket:u32,
    items:Vec<ItemCode>
}

#[derive(Serialize,Deserialize)]
pub struct BountyRefresh{
    pub bbq1:u32,
    pub bbq2:u32,
    pub bbq3:u32,
    pub bbq4:u32,
    pub bbq5:u32,
    pub bbq6:u32,
    pub bbq7:u32,
    pub bbq8:u32,
    pub bbq9:u32,
    pub bbq10:u32,
    pub bbq11:u32,
    pub bbq12:u32,
    pub bbq13:u32,
    pub bbq14:u32,
    pub bbq15:u32,
    pub bbq16:u32,
    pub bbq17:u32,
    pub bbq18:u32,
    pub bbq19:u32,
    pub bbq20:u32,
    pub bbq21:u32,
    pub bbq22:u32,
    pub bbq23:u32,
    pub bbq24:u32,
    pub bbq25:u32,
}
impl BountyRefresh{
    pub fn path()->PathBuf{
        Path::new(".").join("static").join("bounty_refresh.json")
    }
    pub async fn new()->Result<Self,MyErr>{
        let file = tokio::fs::read_to_string(BountyRefresh::path()).await?;
        Ok(serde_json::from_str(&file)?)
    }
    pub async fn save(&self)->Result<(),Error>{
        let string = serde_json::to_string_pretty(&self)?;
        tokio::fs::write(BountyRefresh::path(), string.as_bytes()).await?;
        Ok(())
    }
    pub async fn check(data:&str)->Result<(),Error>{
        let x = serde_json::from_str::<Self>(&data)?;
        Ok(x.save().await?)
    }
}
impl Bounty{
    pub fn path()->PathBuf{
        Path::new(".").join("static").join("bounty.json")
    }
    pub async fn check(data:&str)->Result<(),MyErr>{
        let x = serde_json::from_str::<Self>(&data)?;
        x.save().await?;
        Ok(())
    }
    pub async fn new()->Result<Self,MyErr>{
        let file = tokio::fs::read_to_string(BountyRefresh::path()).await?;
        Ok(serde_json::from_str(&file)?)
    }
    pub async fn save(&self)->Result<(),Error>{
        let string = serde_json::to_string_pretty(&self)?;
        tokio::fs::write(Bounty::path(), string.as_bytes()).await?;
        Ok(())
    }
}
impl Default for BountyDesc{
    fn default() -> Self {
        BountyDesc { description: "this is bbq description".to_owned(), cooldown: 0, solo:BountyReward::default(),multi:BountyReward::default(),
            icon:"https://media.discordapp.net/attachments/1068440173479739393/1068440373132800080/SAF.png".to_owned(),
            thumbnail:"https://media.discordapp.net/attachments/1068440173479739393/1068440373132800080/SAF.png".to_owned(),
            rules:vec!["HR equipment only".to_owned(),"no MS".to_owned(),"naked".to_owned()]
        }
    }
}
impl Default for BountyReward {
    fn default() -> Self {
        BountyReward { coin: 1, ticket: 1, items:vec![ItemCode::default(),ItemCode::default()]}
    }
}
impl Default for BountyBBQ {
    fn default() -> Self {
        BountyBBQ { bbq1: BountyDesc::default(), bbq3: BountyDesc::default(),
        bbq4: BountyDesc::default(), bbq5: BountyDesc::default(),
        bbq2: BountyDesc::default(),
        bbq6: BountyDesc::default(), bbq7: BountyDesc::default(), 
        bbq8: BountyDesc::default(), bbq9: BountyDesc::default(), 
        bbq10: BountyDesc::default(), bbq11: BountyDesc::default(), 
        bbq12: BountyDesc::default(), bbq13: BountyDesc::default(), 
        bbq14: BountyDesc::default(), bbq15: BountyDesc::default(), 
        bbq16: BountyDesc::default(), bbq17: BountyDesc::default(),
        bbq18: BountyDesc::default(), bbq19: BountyDesc::default(),
        bbq20: BountyDesc::default(), bbq21: BountyDesc::default(),
        bbq22: BountyDesc::default(), bbq23: BountyDesc::default(),
        bbq24: BountyDesc::default(), bbq25: BountyDesc::default() }
    }
}
impl Default for Bounty {
    fn default() -> Self {
        Bounty { gold: BountyBBQ::default(), 
        silver: BountyBBQ::default(), 
        bronze: BountyBBQ::default(),
        free: BountyBBQ::default(),
        event: BountyBBQ::default() 
        }
    }
}
impl Default for BountyRefresh {
    fn default() -> Self {
        BountyRefresh { bbq1: 0,bbq2:0, bbq3: 0,
        bbq4: 0, bbq5: 0,
        bbq6: 0, bbq7: 0, 
        bbq8: 0, bbq9: 0, 
        bbq10: 0, bbq11: 0, 
        bbq12: 0, bbq13: 0, 
        bbq14: 0, bbq15: 0, 
        bbq16: 0, bbq17: 0,
        bbq18: 0, bbq19: 0,
        bbq20: 0, bbq21: 0,
        bbq22: 0, bbq23: 0,
        bbq24: 0, bbq25: 0 }
    }
}
#[derive(Serialize,Deserialize,PartialEq, Eq,Clone)]
pub struct TitleImage{
    pub url:String,
    pub diameter:u32,
    pub x_start:u32,
    pub y_start:u32,
}
#[derive(Serialize,Deserialize,PartialEq, Eq,Clone)]
pub struct CustomTitle{
    pub image:TitleImage,
    pub trigger:String,
    pub role_id:u64,
}
#[derive(Serialize,Deserialize,PartialEq, Eq,Clone)]
pub struct BountyTitle{
    pub bronze_bounty:CustomTitle,
    pub silver_bounty:CustomTitle,
    pub gold_bounty:CustomTitle,
    pub bronze_trading:CustomTitle,
    pub silver_trading:CustomTitle,
    pub gold_trading:CustomTitle,
    pub custom:Vec<CustomTitle>
}
impl BountyTitle{
    fn path()->PathBuf{
        Path::new(".").join("static").join("bounty_title.json")
    }
    pub async fn new()->Result<Self,MyErr>{
        let file = tokio::fs::read_to_string(BountyTitle::path()).await?;
        Ok(serde_json::from_str(&file)?)
    }
    pub async fn save(&self)->Result<(),MyErr>{
        let file = serde_json::to_string_pretty(&self)?;
        Ok(tokio::fs::write(BountyTitle::path(), file.as_bytes()).await?)
    }
    pub async fn check(data:&str)->Result<(),MyErr>{
        let x = serde_json::from_str::<Self>(data)?;
        x.save().await?;
        Ok(())
    }
}
impl Default for TitleImage {
    fn default() -> Self {
        TitleImage { url: "https://media.discordapp.net/attachments/1004207525408817323/1006334931607232542/01._Bounty_Expert.jpg?width=1150&height=658".to_owned(),
        diameter: 180, x_start: 695, y_start: 170 }
    }
}
impl Default for CustomTitle {
    fn default() -> Self {
        CustomTitle { image: TitleImage::default(),
        trigger: "4_0".to_owned(), role_id: 1031595216538452038 }
    }
}
impl Default for BountyTitle {
    fn default() -> Self {
        BountyTitle {custom: vec![CustomTitle::default(),CustomTitle::default()],
        bronze_bounty:CustomTitle::default(),silver_bounty:CustomTitle::default(),gold_bounty:CustomTitle::default(),
        bronze_trading:CustomTitle::default(),silver_trading:CustomTitle::default(),gold_trading:CustomTitle::default(),
        }
    }
}

#[cfg(test)]
mod testing{
    use super::*;
    #[tokio::test]
    async fn get_json() {
        let x = Bounty::default();
        let y = BountyRefresh::default();
        let z = BountyTitle::default();
        x.save().await.unwrap();
        y.save().await.unwrap();
        z.save().await.unwrap();
    }
}
