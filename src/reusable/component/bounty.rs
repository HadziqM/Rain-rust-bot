use std::path::Path;
use std::path::PathBuf;

use serenity::all::*;
use crate::BOUNTY;
use crate::event::interaction::SlashBundle;
use crate::material::ItemPedia;
use crate::reusable::utils::Color;
use super::MyErr;
use crate::reusable::postgress::card::Event;
#[derive(Clone)]
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

#[derive(Clone)]
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
    pub fn encode(&self)->u8{
        match self {
            Self::Solo => 0,
            Self::Multi => 1,
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
    pub fn get_reward<'a>(&self,bnt:&'a BountyDesc)->&'a BountyReward{
        match self{
            Self::Solo=>&bnt.solo,
            Self::Multi=>&bnt.multi
        }
    }
}


#[derive(PartialEq, Eq,Clone)]  
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
    pub fn encode(&self)->u8{
        match self{
            Category::Bronze=>0,
            Category::Silver=>1,
            Category::Gold=>2,
            Category::Free=>3,
            Category::Event=>4,
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
    fn rank(&self,event:&Event)->i32{
        let idk = match self{
            Self::Gold=>event.gold + 1,
            Self::Bronze=>event.bronze + 1,
            Self::Silver=>event.silver + 1,
            _ => 50
        };
        idk
    }
    fn check_rank(&self,event:&Event,code:u8)->bool{
        if code as i32> self.rank(event){
            return false;
        }
        true
    }
    fn change_rank(&self,hunt:&mut Hunter,code:u8){
        if code as i32 == self.rank(&hunt.event){
            match self {
                Self::Gold=>{
                    hunt.event.gold += 1;
                }
                Self::Silver=>{
                    hunt.event.silver += 1;
                }
                Self::Bronze=>{
                    hunt.event.bronze += 1
                }
                _ =>{return;}
            }
        }
    }
    fn bounty_path(&self)->PathBuf{
        let x = Path::new(".").join("static");
        match self{
            Self::Gold=>x.join("gold_bounty.json"),
            Self::Silver=>x.join("silver_bounty.json"),
            Self::Bronze=>x.join("bronze_bounty.json"),
            Self::Free=>x.join("free_bounty.json"),
            Self::Event=>x.join("event_bounty.json"),
        }
    }
}


#[derive(PartialEq, Eq,Clone)]
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
            1=>Ok(BBQ::BBQ01),
            2=>Ok(BBQ::BBQ02),
            3=>Ok(BBQ::BBQ03),
            4=>Ok(BBQ::BBQ04),
            5=>Ok(BBQ::BBQ05),
            6=>Ok(BBQ::BBQ06),
            7=>Ok(BBQ::BBQ07),
            8=>Ok(BBQ::BBQ08),
            9=>Ok(BBQ::BBQ09),
            10=>Ok(BBQ::BBQ10),
            11=>Ok(BBQ::BBQ11),
            12=>Ok(BBQ::BBQ12),
            13=>Ok(BBQ::BBQ13),
            14=>Ok(BBQ::BBQ14),
            15=>Ok(BBQ::BBQ15),
            16=>Ok(BBQ::BBQ16),
            17=>Ok(BBQ::BBQ17),
            18=>Ok(BBQ::BBQ18),
            19=>Ok(BBQ::BBQ19),
            20=>Ok(BBQ::BBQ20),
            21=>Ok(BBQ::BBQ21),
            22=>Ok(BBQ::BBQ22),
            23=>Ok(BBQ::BBQ23),
            24=>Ok(BBQ::BBQ24),
            25=>Ok(BBQ::BBQ25),
            _=>Err(MyErr::Custom("cant get bbq".to_string()))
        }
    }
    pub fn name_encode(code:u8)->String{
        match Self::new(code){
            Ok(x)=>x.name(),
            Err(_)=>"Not Progressed Yet".to_owned()
        }
    }
    pub fn encode(&self)->u8{
        match self{
            BBQ::BBQ01=>1,
            BBQ::BBQ02=>2,
            BBQ::BBQ03=>3,
            BBQ::BBQ04=>4,
            BBQ::BBQ05=>5,
            BBQ::BBQ06=>6,
            BBQ::BBQ07=>7,
            BBQ::BBQ08=>8,
            BBQ::BBQ09=>9,
            BBQ::BBQ10=>10,
            BBQ::BBQ11=>11,
            BBQ::BBQ12=>12,
            BBQ::BBQ13=>13,
            BBQ::BBQ14=>14,
            BBQ::BBQ15=>15,
            BBQ::BBQ16=>16,
            BBQ::BBQ17=>17,
            BBQ::BBQ18=>18,
            BBQ::BBQ19=>19,
            BBQ::BBQ20=>20,
            BBQ::BBQ21=>21,
            BBQ::BBQ22=>22,
            BBQ::BBQ23=>23,
            BBQ::BBQ24=>24,
            BBQ::BBQ25=>25,
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
            ("1".to_owned(),"BBQ01".to_owned()),
            ("2".to_owned(),"BBQ02".to_owned()),
            ("3".to_owned(),"BBQ03".to_owned()),
            ("4".to_owned(),"BBQ04".to_owned()),
            ("5".to_owned(),"BBQ05".to_owned()),
            ("6".to_owned(),"BBQ06".to_owned()),
            ("7".to_owned(),"BBQ07".to_owned()),
            ("8".to_owned(),"BBQ08".to_owned()),
            ("9".to_owned(),"BBQ09".to_owned()),
            ("10".to_owned(),"BBQ10".to_owned()),
            ("11".to_owned(),"BBQ11".to_owned()),
            ("12".to_owned(),"BBQ12".to_owned()),
            ("13".to_owned(),"BBQ13".to_owned()),
            ("14".to_owned(),"BBQ14".to_owned()),
            ("15".to_owned(),"BBQ15".to_owned()),
            ("16".to_owned(),"BBQ16".to_owned()),
            ("17".to_owned(),"BBQ17".to_owned()),
            ("18".to_owned(),"BBQ18".to_owned()),
            ("19".to_owned(),"BBQ19".to_owned()),
            ("20".to_owned(),"BBQ20".to_owned()),
            ("21".to_owned(),"BBQ21".to_owned()),
            ("22".to_owned(),"BBQ22".to_owned()),
            ("23".to_owned(),"BBQ23".to_owned()),
            ("24".to_owned(),"BBQ24".to_owned()),
            ("25".to_owned(),"BBQ25".to_owned()),
        ]
    }
    pub fn get_bounty<'a>(&self,bbq:&'a Bounty)->Result<&'a BountyDesc,MyErr>{
        for i in &bbq.bounty{
            if i.bbq == self.encode(){
                return Ok(i);
            }
        }
        Err(MyErr::Custom("the jsont inst configured yet".to_owned()))
    }
}
#[derive(Clone)]
pub struct Hunter{
    pub member:Member,
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
use super::Components;
use super::market::Market;
use crate::PgConn;
use crate::reusable::utils::MyTime;
impl Hunter {
    async fn new(bnd:&SlashBundle<'_>,bypass:bool,user:Member,uid:Vec<UserId>,pg:&mut PgConn<'_>,bbq:&BBQ,category:&Category)->Result<Vec<Hunter>,MyErr>{
        let mut vec = vec![user];
        let mut hunter = Vec::new();
        let guild = bnd.cmd.guild_id.unwrap().to_partial_guild(&bnd.ctx.http).await?;
        for i in &uid{
            let mem = guild.member(&bnd.ctx.http, i.to_owned()).await?;
            vec.push(mem);
        }
        for us in vec{
            pg.did = us.user.id.to_string();
            let event = pg.get_event().await
                .ok().ok_or(MyErr::Custom(format!("{} most likely isnt binded yet please check",us.to_string())))?;
            let title = Title::new(u8::try_from(event.title).unwrap());
            if !bypass{
                if let Some((x,y))=BountyTitle::decrypt(&event.latest_bounty){
                    let time;
                    if &x==category && &y==bbq{
                        time = event.latest_bounty_time + 40*60*60;
                    }else {
                        time = event.latest_bounty_time + 20*60*60;
                    }
                    if time > MyTime::now() && *category != Category::Event{
                        return Err(MyErr::Custom(format!("{} still on cooldown till <t:{time}:R>",us.to_string())));
                    }
                }
                let sub = bbq.encode();
                if !category.check_rank(&event,sub){
                    return Err(MyErr::Custom(format!("{} cant take BBQ higher than {} for category {}"
                        ,us.to_string(),BBQ::new(category.rank(&event) as u8)?.name(),category.name())));
                }

            }
            hunter.push(Hunter{member:us.to_owned(),title,event});
        }
        Ok(hunter)
    }
}

use std::collections::HashMap;
use crate::Mybundle;
impl BountySubmit{
    pub async fn new(bnd:&SlashBundle<'_>,bypass:bool,user:Member,uid:Vec<UserId>,pg:&mut PgConn<'_>,bounty:&Box<Bounty>,url:&str,method:Methode,bbq:BBQ,category:Category)
        ->Result<BountySubmit,MyErr>{
        let hunter = Hunter::new(bnd,bypass,user, uid, pg,&bbq,&category).await?;
        let desc = bbq.get_bounty(*&bounty)?;
        let reward = method.get_reward(desc).clone();
        Ok(BountySubmit{hunter,method,bbq,category,url:url.to_owned(),thumb:desc.icon.to_owned(),reward,time:MyTime::now()})
    }
    async fn send_reward(&self,pg:&mut PgConn<'_>)->Result<(),MyErr>{
        let name = format!("{} {} {} Rewards",self.method.name(),self.category.name(),self.bbq.name());
        let desc = format!("Reward for clearing server event {} {} as {}",self.category.name(),self.bbq.name(),self.method.name());
        for i in &self.hunter{
            pg.did = i.member.user.id.to_string();
            pg.send_item(&self.reward.items.as_slice(), i.event.char_id, &name, &desc).await?;
            pg.bounty_event(&i.event).await?;
        }
        Ok(())
    }
    pub async fn reward<T:Mybundle>(&mut self,bypass:bool,bnd:&T,pg:&mut PgConn<'_>)->Result<(),MyErr>{
        for hunt in self.hunter.iter_mut(){
            pg.did = hunt.member.user.id.to_string();
            let eve = pg.get_event().await?;
            hunt.event.bounty = eve.bounty;
            hunt.event.gacha = eve.gacha;
            let bounty = self.reward.coin as f32 * hunt.title.bonus();
            hunt.event.bounty += bounty as i32;
            hunt.event.gacha += self.reward.ticket as i32;
            self.category.change_rank(hunt, self.bbq.encode());
            hunt.event.latest_bounty = BountyTitle::encrypt(self.category.clone(), self.bbq.clone());
            if !bypass{
                hunt.event.latest_bounty_time = self.time;
            }
        }
        self.send_reward(pg).await?;
        let ch = ChannelId::new(bnd.init().bounty.receptionist_ch);
        for (user,embed) in self.reward_embed(bnd.pedia()){
            ch.send_message(&bnd.ctx().http, CreateMessage::new().embed(embed)
                .content(format!("{}'s {} {} {} Reward already distributed"
                    ,user.to_string(), self.method.name(),self.category.name()
                    ,self.bbq.name()))).await?;
        }
        Ok(())
    }
    pub async fn title<T:Mybundle>(&mut self,bnd:&T,title:&BountyTitle)->Result<(),MyErr>{
        let code = BountyTitle::encrypt(self.category.clone(), self.bbq.clone());
        for hunt in self.hunter.iter_mut(){
            title.add_title(bnd, &mut hunt.event, &mut hunt.member, &code).await?;
        }
        Ok(())
    }
    pub async fn save(&self,did:&str){
        let mut bounty = BOUNTY.lock().await;
        match bounty.get_mut(did){
            Some(x)=>{
                *x = self.clone();
            }
            None=>{
                bounty.insert(did.to_owned(), self.clone());
            }
        }
    }
    pub async fn open(did:&str)->Option<Self>{
        BOUNTY.lock().await.get(did).cloned()
    }
    pub async fn delete(&self){
        BOUNTY.lock().await.remove(&self.hunter[0].member.user.id.to_string());
    }
    pub fn embed(&self)->CreateEmbed{
        let title = format!("{} {} {}",self.category.name(),self.bbq.name(),self.method.name());
        let mut desc = Vec::new();
        for i in &self.hunter{
            desc.push("\n".to_owned());
            desc.push(format!("name : {} > {}",i.member.user.to_string(),&i.event.name));
        }
        let res = format!("Submitted At <t:{}:F>\n{}",self.time,desc[1..].concat());
        let author = &self.hunter[0].member;
        CreateEmbed::new().title(title).description(res).thumbnail(&self.thumb).image(&self.url)
            .author(CreateEmbedAuthor::new(&author.user.name).url(author.face())).color(self.category.color().throw())
    }
    pub fn button(&self)->Vec<CreateActionRow>{
        let accept = Components::normal_button("Approve", &format!("bounty_{}_a",self.hunter[0].member.user.id.to_string()), ButtonStyle::Primary, "👌");
        let reject = Components::normal_button("Reject", &format!("bounty_{}_r",self.hunter[0].member.user.id.to_string()), ButtonStyle::Danger, "👎");
        let arow = CreateActionRow::Buttons(vec![accept,reject]);
        vec![arow]
    }
    pub fn cooldown(&self,bnt:&mut Box<BountyRefresh>,)->bool{
        for (bbq,cd) in bnt.hashmap(){
            if cd != 0 && bbq == self.bbq {
                if self.category == Category::Free{
                    bnt.set_cd(&bbq, cd-1);
                }
                return true;
            }
        }
        false
    }
    fn reward_embed<'a>(&'a self,item:&ItemPedia)->HashMap<&'a User,CreateEmbed>{
        let mut out = HashMap::new();
        let mut reward = vec![">>> ".to_owned()];
        let ticket = format!("Ticket Reward: {} Ticket(s)"
            ,self.reward.ticket);
        for i in &self.reward.items{
            reward.push(i.text(item).unwrap());
            reward.push("\n".to_owned());
        }
        let lenre = reward.len()-1;
        for j in &self.hunter{
            let bon = (j.title.bonus()-1.0)*100.0;
            let percent = format!("{}%",bon as u32);
            let bonus = self.reward.coin as f64 * j.title.bonus() as f64;
            let bounty =  format!("Bounty Reward: {}\nTitle Bonus: {}\nReward With Bonus: {}"
                ,Market::currency(self.reward.coin as i64),percent,Market::currency(bonus as i64));
            let embed = CreateEmbed::new().title("Bounty Reward")
                .description(format!("{}'s reward for {} {} category {}",j.member.to_string()
                    ,self.method.name(),self.bbq.name(),self.category.name()))
                .fields(vec![
                        ("Bounty Coin",bounty,false),
                        ("Gacha Tickets",ticket.to_owned(),false),
                        ("Items/Equipment",reward[..lenre].to_vec().concat(),false),
                ]).color(Color::Gold.throw())
                .author(CreateEmbedAuthor::new(&j.member.user.name).icon_url(j.member.user.face()));
            out.insert(&j.member.user, embed);
        }
        out
    }
}

use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,PartialEq, Eq)]
pub struct Bounty{
    pub bounty:[BountyDesc; 25],
}
use super::super::bitwise::ItemCode;
#[derive(Serialize,Deserialize,PartialEq, Eq,Clone,Debug)]
pub struct BountyDesc {
    pub bbq:u8,
    pub description:String,
    pub cooldown:u32,
    pub icon:String,
    pub thumbnail:String,
    pub rules:Vec<String>,
    pub solo:BountyReward,
    pub multi:BountyReward
}
#[derive(Serialize,Deserialize,PartialEq, Eq,Clone,Debug)]
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
    pub fn path2()->PathBuf{
        Path::new(".").join("static").join("cooldown.json")
    }
    pub async fn new(coodown:bool)->Result<Self,MyErr>{
        let path = || {if coodown{BountyRefresh::path2()}else{BountyRefresh::path()}};
        let file = tokio::fs::read_to_string(path()).await?;
        Ok(serde_json::from_str(&file)?)
    }
    pub async fn save(&self,coodown:bool)->Result<(),MyErr>{
        let path = || {if coodown{BountyRefresh::path2()}else{BountyRefresh::path()}};
        let string = serde_json::to_string_pretty(&self)?;
        tokio::fs::write(path(), string.as_bytes()).await?;
        Ok(())
    }
    pub async fn check(data:&[u8])->Result<(),MyErr>{
        let x = serde_json::from_slice::<Self>(&data)?;
        Ok(x.save(false).await?)
    }
    pub async fn rejected<T:Mybundle>(bbq:&BBQ,bnd:&T)->Result<(),MyErr>{
        let mut x = Self::new(true).await?;
        for (qq,cd) in x.hashmap(){
            if bbq == &qq{
                x.set_cd(bbq, cd+1);
            }
        }
        x.save(true).await?;
        x.cooldown(bnd).await
    }
    pub async fn cooldown<T:Mybundle>(&self,bnd:&T)->Result<(),MyErr>{
        let mut msg = ChannelId::new(bnd.init().bounty.cooldown_ch)
            .message(&bnd.ctx().http, MessageId::new(bnd.init().bounty.cooldown_msg)).await?;
        msg.edit(&bnd.ctx().http, EditMessage::new().embed(self.cooldown_embed())).await?;
        Ok(())
    }
    pub fn set_cd(&mut self,bbq:&BBQ,cd:u32){
        match bbq{
            BBQ::BBQ01=>{self.bbq1=cd}
            BBQ::BBQ02=>{self.bbq2=cd}
            BBQ::BBQ03=>{self.bbq3=cd}
            BBQ::BBQ04=>{self.bbq4=cd}
            BBQ::BBQ05=>{self.bbq5=cd}
            BBQ::BBQ06=>{self.bbq6=cd}
            BBQ::BBQ07=>{self.bbq7=cd}
            BBQ::BBQ08=>{self.bbq8=cd}
            BBQ::BBQ09=>{self.bbq9=cd}
            BBQ::BBQ10=>{self.bbq10=cd}
            BBQ::BBQ11=>{self.bbq11=cd}
            BBQ::BBQ12=>{self.bbq12=cd}
            BBQ::BBQ13=>{self.bbq13=cd}
            BBQ::BBQ14=>{self.bbq14=cd}
            BBQ::BBQ15=>{self.bbq15=cd}
            BBQ::BBQ16=>{self.bbq16=cd}
            BBQ::BBQ17=>{self.bbq17=cd}
            BBQ::BBQ18=>{self.bbq18=cd}
            BBQ::BBQ19=>{self.bbq19=cd}
            BBQ::BBQ20=>{self.bbq20=cd}
            BBQ::BBQ21=>{self.bbq21=cd}
            BBQ::BBQ22=>{self.bbq22=cd}
            BBQ::BBQ23=>{self.bbq23=cd}
            BBQ::BBQ24=>{self.bbq24=cd}
            BBQ::BBQ25=>{self.bbq25=cd}
        }
    }
    fn cooldown_embed(&self)->CreateEmbed{
        let mut idk = "```".to_owned();
        for (bbq,cd) in &self.hashmap(){
            idk.push_str(&format!("\n{} ======= {} left",bbq.name(),cd));
        }
        idk.push_str("\n```");
        CreateEmbed::new().title("Free Category Cooldown").description(idk)
            .color(Color::Random.throw())
    }
    fn hashmap(&self)->[(BBQ,u32); 25]{
        [
            (BBQ::BBQ01,self.bbq1),
            (BBQ::BBQ02,self.bbq2),
            (BBQ::BBQ03,self.bbq3),
            (BBQ::BBQ04,self.bbq4),
            (BBQ::BBQ05,self.bbq5),
            (BBQ::BBQ06,self.bbq6),
            (BBQ::BBQ07,self.bbq7),
            (BBQ::BBQ08,self.bbq8),
            (BBQ::BBQ09,self.bbq9),
            (BBQ::BBQ10,self.bbq10),
            (BBQ::BBQ11,self.bbq11),
            (BBQ::BBQ12,self.bbq12),
            (BBQ::BBQ13,self.bbq13),
            (BBQ::BBQ14,self.bbq14),
            (BBQ::BBQ15,self.bbq15),
            (BBQ::BBQ16,self.bbq16),
            (BBQ::BBQ17,self.bbq17),
            (BBQ::BBQ18,self.bbq18),
            (BBQ::BBQ19,self.bbq19),
            (BBQ::BBQ20,self.bbq20),
            (BBQ::BBQ21,self.bbq21),
            (BBQ::BBQ22,self.bbq22),
            (BBQ::BBQ23,self.bbq23),
            (BBQ::BBQ24,self.bbq24),
            (BBQ::BBQ25,self.bbq25),
        ]
    }
}
impl Bounty{
    pub fn path(category:&Category)->PathBuf{
        category.bounty_path()
    }
    pub fn desc<T:Mybundle>(&self,bnd:&T,bbq:&BBQ,category:&Category)->Result<CreateEmbed,MyErr>{
        let pedia = bnd.pedia();
        let bdesc = bbq.get_bounty(&self)?;
        let mut rules = vec![">>> ".to_owned()];
        let mut solo = vec![">>> ".to_owned()];
        let mut multi = vec![">>> ".to_owned()];
        for (i,v) in bdesc.rules.iter().enumerate(){
            rules.push(format!("{}. {v}",i+1));
            rules.push("\n".to_owned());
        }
        for i in &bdesc.solo.items{
            solo.push(i.text(pedia).unwrap());
            solo.push("\n".to_owned());
        }
        for i in &bdesc.multi.items{
            multi.push(i.text(pedia).unwrap());
            multi.push("\n".to_owned());
        }
        let rulen = rules.len() -1;
        let mulen = multi.len() -1;
        let solen = solo.len() -1;
        Ok(CreateEmbed::new().title(format!("{} {}",category.name(),bbq.name()))
            .description(&bdesc.description)
            .thumbnail(&bdesc.icon).image(&bdesc.thumbnail)
            .field("Rules", rules[..rulen].concat(), false)
            .field("Solo Rewards", format!("Bounty Coin: {}\nGacha Ticket: {} Ticket\n{}"
                ,Market::currency(bdesc.solo.coin as i64),bdesc.solo.ticket,solo[..solen].concat()), false)
            .field("Multiplayer Rewards", format!("Bounty Coin: {}\nGacha Ticket: {} Ticket\n{}"
                ,Market::currency(bdesc.multi.coin as i64),bdesc.multi.ticket,multi[..mulen].concat()), false)
            .color(Color::Random.throw()))
    }
    pub async fn check(data:&[u8],category:&Category)->Result<(),MyErr>{
        let x = serde_json::from_slice::<Self>(&data)?;
        let pedia = ItemPedia::default();
        for i in &x.bounty{
            let mut item = i.solo.items.clone();
            item.append(&mut i.multi.items.clone());
            for it in item{
                if !it.check(&pedia){
                    return Err(MyErr::Custom(format!("There is item missmatch on 
                        data struct\n`{it:?}`\nthose item isnt exist on database")));
                }
            }
        }
        x.save(category).await?;
        Ok(())
    }
    pub async fn new(category:&Category)->Result<Self,MyErr>{
        let file = tokio::fs::read(Bounty::path(category)).await?;
        Ok(serde_json::from_slice(&file)?)
    }
    pub async fn save(&self,category:&Category)->Result<(),Error>{
        let string = serde_json::to_string_pretty(&self)?;
        tokio::fs::write(Bounty::path(category), string.as_bytes()).await?;
        Ok(())
    }
}
fn add_0(num:u8)->String{
    if num < 10{
        return format!("0{num}");
    }
    num.to_string()
}
impl Default for BountyDesc{
    fn default() -> Self {
        BountyDesc { description: "this is bbq description".to_owned(), cooldown: 0,bbq:0, solo:BountyReward::default(),multi:BountyReward::default(),
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
impl Default for Bounty {
    fn default() -> Self {
        let x:[BountyDesc; 25] = (0..25)
            .map(|_|BountyDesc::default())
            .collect::<Vec<_>>().try_into().unwrap();
        Bounty { bounty: x}
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
    pub db_code:u8
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
impl CustomTitle{
    pub fn embed(&self,user:&User)->CreateEmbed{
        let color = BountyTitle::decrypt(&self.trigger).unwrap().0.color().throw();
        CreateEmbed::new().title("Congratulation On Promotion").color(color)
            .description(format!("You got Role <@&{}> after Clearing {}",self.role_id,BountyTitle::name(&self.trigger)))
            .image("attachment://title.jpg").author(CreateEmbedAuthor::new(&user.name).icon_url(user.face()))
    }
    pub async fn send_title<T:Mybundle>(&self,bnd:&T,user:&User)->Result<(),MyErr>{
        let image = self.image.title(&user.static_avatar_url().unwrap_or(user.default_avatar_url())).await?;
        ChannelId::new(bnd.init().bounty.promotion_ch)
            .send_message(&bnd.ctx().http, CreateMessage::new().add_file(CreateAttachment::bytes(image, "title.jpg"))
                .embed(self.embed(user))
                .content(format!("for {}",user.to_string()))).await?;
        Ok(())
    }
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
    pub async fn check(data:&[u8])->Result<(),MyErr>{
        let x = serde_json::from_slice::<Self>(data)?;
        for i in &x.custom{
            if i.db_code != 0{
                return Err(MyErr::Custom("you cant set value other than 0 for custom title's db_code".to_owned()));
            }
        }
        x.save().await?;
        Ok(())
    }
    pub fn hash<'a>(&'a self)->HashMap<&'a str,&'a CustomTitle>{
        let mut images:HashMap<&str,&CustomTitle> = HashMap::new();
        for (i,_) in self.custom.iter().enumerate(){
            images.insert(&self.custom[i].trigger,&self.custom[i]);
        }
        images.insert(&self.bronze_bounty.trigger,&self.bronze_bounty);
        images.insert(&self.silver_bounty.trigger,&self.silver_bounty);
        images.insert(&self.gold_bounty.trigger,&self.gold_bounty);
        images.insert(&self.bronze_trading.trigger,&self.bronze_trading);
        images.insert(&self.silver_trading.trigger,&self.silver_trading);
        images.insert(&self.gold_trading.trigger,&self.gold_trading);
        images
    }
    pub async fn add_title<T:Mybundle>(&self,bnd:&T,event:&mut Event,member:&mut Member,trigger:&str)->Result<(),MyErr>{
        for i in self.get_trigger(){
            let role = RoleId::new(i.role_id);
            if i.trigger.as_str()==trigger{
                if i.db_code!= 0{
                    if !Title::matching(event.title as u8, i.db_code){
                        event.title += i.db_code as i32;
                    }else {
                        return Ok(());
                    }
                }else{ 
                    if member.roles.contains(&role){
                        return Ok(());
                    }
                }
                if !member.roles.contains(&role){
                    member.add_role(&bnd.ctx().http, role).await?;
                }
                i.send_title(bnd, &member.user).await?;
            } 
        }
        Ok(())
    }
    fn decrypt(code:&str)->Option<(Category,BBQ)>{
        let mut x = code.split("_");
        let cat =Category::new(x.next()?.parse::<u8>().ok()?).ok()?;
        let bbq = BBQ::new(x.next()?.parse::<u8>().ok()?).ok()?;
        Some((cat,bbq))
    }
    fn encrypt(category:Category,bbq:BBQ)->String{
        format!("{}_{}",category.encode(),bbq.encode())
    }
    fn get_trigger<'a>(&'a self)->Vec<&'a CustomTitle>{
        let mut out = vec![
            &self.silver_bounty,
            &self.bronze_bounty,
            &self.gold_bounty,
            &self.silver_trading,
            &self.gold_trading,
            &self.bronze_trading,
        ];
        for i in &self.custom{
            out.push(i);
        }
        out
    }
    pub fn name(code:&str)->String{
        match Self::decrypt(code){
            Some((x,y))=>format!("{} {}",x.name(),y.name()),
            None=>String::from(code)
        }
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
        trigger: "4_0".to_owned(), role_id: 1031595216538452038,db_code:0 }
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
    #[ignore = "already have"]
    async fn get_json() {
        let x = Bounty::default();
        let y = BountyRefresh::default();
        let z = BountyTitle::default();
        x.save(&Category::Free).await.unwrap();
        y.save(false).await.unwrap();
        z.save().await.unwrap();
    }
    // #[tokio::test]
    // #[ignore = "already tested"]
    // async fn refresh() {
    //     let mut x = Bounty::default();
    //     let y = BountyRefresh::new(false).await.unwrap();
    //     x.save(&Category::Free).await.unwrap();
    // }
    // #[tokio::test]
    // #[ignore = "already tested"]
    // async fn reset_cd() {
    //     let mut x = Bounty::new(&Category::Free).await.unwrap();
    //     assert_eq!(x.bounty[0].cooldown,100);
    //     x.save(&Category::Free).await.unwrap();
    // }
    // #[tokio::test]
    // async fn overflow() {
    //     let refresh = BountyRefresh::new(false).await.unwrap();
    //     let mut bounty = Bounty::new(&Category::Free).await.unwrap();
    //     bounty.save(&Category::Free).await.unwrap();
    // }
}
