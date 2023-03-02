use std::path::Path;
use std::path::PathBuf;

use serenity::all::*;
use crate::BOUNTY;
use crate::event::interaction::SlashBundle;
use crate::material::ItemPedia;
use crate::reusable::utils::Color;
use super::MyErr;
use std::cell::RefCell;
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
    fn get_reward<'a>(&self,bnt:&'a BountyDesc)->&'a BountyReward{
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
    pub fn encode(self)->u8{
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
    fn get_bounty<'a>(&self,bounty:&'a Bounty)->&'a BountyBBQ{
        match self{
            Self::Gold=>&bounty.gold,
            Self::Silver=>&bounty.silver,
            Self::Bronze=>&bounty.bronze,
            Self::Free=>&bounty.free,
            Self::Event=>&bounty.event
        }
    }
    fn rank(&self,event:&Event)->i32{
        let idk = match self{
            Self::Gold=>event.gold + 1,
            Self::Bronze=>event.bronze + 1,
            Self::Silver=>event.silver + 1,
            _ => 0
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
    pub fn encode(&self)->u8{
        match self{
            BBQ::BBQ01=>0,
            BBQ::BBQ02=>1,
            BBQ::BBQ03=>2,
            BBQ::BBQ04=>3,
            BBQ::BBQ05=>4,
            BBQ::BBQ06=>5,
            BBQ::BBQ07=>6,
            BBQ::BBQ08=>7,
            BBQ::BBQ09=>8,
            BBQ::BBQ10=>9,
            BBQ::BBQ11=>10,
            BBQ::BBQ12=>11,
            BBQ::BBQ13=>12,
            BBQ::BBQ14=>13,
            BBQ::BBQ15=>14,
            BBQ::BBQ16=>15,
            BBQ::BBQ17=>16,
            BBQ::BBQ18=>17,
            BBQ::BBQ19=>18,
            BBQ::BBQ20=>19,
            BBQ::BBQ21=>20,
            BBQ::BBQ22=>21,
            BBQ::BBQ23=>22,
            BBQ::BBQ24=>23,
            BBQ::BBQ25=>24,
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
    async fn new(bnd:&SlashBundle<'_>,bypass:bool,user:Member,msg:&Message,pg:&mut PgConn<'_>,bbq:&BBQ,category:&Category)->Result<Vec<Hunter>,MyErr>{
        let mut vec = vec![user];
        let mut hunter = Vec::new();
        let guild = bnd.cmd.guild_id.unwrap().to_partial_guild(&bnd.ctx.http).await?;
        for i in &msg.mentions{
            let mem = guild.member(&bnd.ctx.http, i.id).await?;
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
                let tresh = bbq.encode() +1;
                if !category.check_rank(&event,tresh){
                    return Err(MyErr::Custom(format!("{} cant take BBQ higher than {} for category {}"
                        ,us.to_string(),BBQ::new(tresh)?.name(),category.name())));
                }

            }
            hunter.push(Hunter{member:us.to_owned(),title,event});
        }
        Ok(hunter)
    }
}

use std::collections::HashMap;
impl BountySubmit{
    pub async fn new(bnd:&SlashBundle<'_>,bypass:bool,user:Member,msg:&Message,pg:&mut PgConn<'_>,bounty:&Bounty,url:&str,method:Methode,bbq:BBQ,category:Category)
        ->Result<BountySubmit,MyErr>{
        let hunter = Hunter::new(bnd,bypass,user, msg, pg,&bbq,&category).await?;
        let idk = category.get_bounty(bounty);
        let desc = bbq.get_bounty(idk);
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
    pub async fn reward(&mut self,bypass:bool,bnd:&SlashBundle<'_>,pg:&mut PgConn<'_>)->Result<(),MyErr>{
        for hunt in self.hunter.iter_mut(){
            let bounty = self.reward.coin as f32 * hunt.title.bonus();
            hunt.event.bounty += bounty as i32;
            hunt.event.gacha += self.reward.ticket as i32;
            self.category.change_rank(hunt, self.bbq.encode() + 1);
            if self.category != Category::Event{
                hunt.event.latest_bounty = BountyTitle::encrypt(self.category.clone(), self.bbq.clone());
                if !bypass{
                    hunt.event.latest_bounty_time = self.time;
                }
            }
        }
        self.send_reward(pg).await?;
        let ch = ChannelId::new(bnd.init.bounty.receptionist_ch);
        for (user,embed) in self.reward_embed(bnd.pedia){
            ch.send_message(&bnd.ctx.http, CreateMessage::new().embed(embed)
                .content(format!("{}'s {} {} {} Reward already distributed"
                    ,user.to_string(), self.method.name(),self.category.name()
                    ,self.bbq.name()))).await?;
        }
        Ok(())
    }
    pub async fn title(&mut self,bnd:&SlashBundle<'_>,title:&BountyTitle)->Result<(),MyErr>{
        let code = BountyTitle::encrypt(self.category.clone(), self.bbq.clone());
        for hunt in self.hunter.iter_mut(){
            for i in title.get_trigger(){
                if code == i.trigger{
                    if i.db_code != 0{
                        if Title::matching(hunt.event.title as u8, i.db_code){
                            hunt.event.title += i.db_code as i32;
                            hunt.title = Title::new(hunt.event.title as u8);
                        }else {
                            return Ok(());
                        }
                    }else{
                        if hunt.member.roles.contains(&RoleId::new(i.role_id)){
                            return Ok(());
                        }
                    }
                    hunt.member.add_role(&bnd.ctx.http, RoleId::new(i.role_id)).await?;
                    i.send_title(bnd, &hunt.member.user).await?;
                }
            }
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
    pub async fn delete(did:&str){
        BOUNTY.lock().await.remove(did);
    }
    pub fn embed(&self)->CreateEmbed{
        let title = format!("{} {} {}",self.category.name(),self.bbq.name(),self.method.name());
        let mut desc = Vec::new();
        for i in &self.hunter{
            desc.push("\n".to_owned());
            desc.push(format!("```\nname\t:\t{}\ndiscord\t:\t{}\n```",&i.event.name,&i.member.user.name));
        }
        let res = format!("Submitted At <t:{}:F>\n{}",self.time,desc[1..].concat());
        let author = &self.hunter[0].member;
        CreateEmbed::new().title(title).description(res).thumbnail(&self.thumb).image(&self.url)
            .author(CreateEmbedAuthor::new(&author.user.name).url(author.face())).color(self.category.color().throw())
    }
    pub fn button(&self)->Vec<CreateActionRow>{
        let accept = Components::normal_button("Approve", "bounty_a", ButtonStyle::Primary, "👌");
        let reject = Components::normal_button("Reject", "bounty_r", ButtonStyle::Danger, "👎");
        let arow = CreateActionRow::Buttons(vec![accept,reject]);
        vec![arow]
    }
    pub fn cooldown(&self,bnt:&mut Bounty)->bool{
        if self.category != Category::Free{
            return true;
        }
        let dec = self.bbq.get_bounty(self.category.get_bounty(bnt));
        if dec.cooldown > RefCell::new(0) {
            dec.cooldown.replace_with(|&mut x|x-1);
            return true;
        }
        false
    }
    pub fn reward_embed<'a>(&'a self,item:&ItemPedia)->HashMap<&'a User,CreateEmbed>{
        let mut out = HashMap::new();
        let mut reward = Vec::new();
        let ticket = format!("Ticket Reward: {} Ticket(s)"
            ,self.reward.ticket);
        for i in &self.reward.items{
            reward.push("/n".to_owned());
            reward.push(i.text(item).unwrap());
        }
        for j in &self.hunter{
            let bon = j.title.bonus()*100.0;
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
                        ("Items/Equipment",reward[1..].to_vec().concat(),false),
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
    pub cooldown:RefCell<u32>,
    pub icon:String,
    pub thumbnail:String,
    pub rules:Vec<String>,
    pub solo:BountyReward,
    pub multi:BountyReward
}
#[derive(Serialize,Deserialize,PartialEq, Eq,Clone)]
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
        let pedia = ItemPedia::default();
        for i in &x.get_items(){
            if !i.check(&pedia){
                return Err(MyErr::Custom(format!("There is item missmatch on 
                    data struct\n`{i:?}`\nthose item isnt exist on database")));
            }
        }
        x.save().await?;
        Ok(())
    }
    fn get_items(&self)->Vec<ItemCode>{
        let mut out = Vec::new();
        out.append(&mut self.gold.bbq1.solo.items.clone());
        out.append(&mut self.gold.bbq1.multi.items.clone());
        out.append(&mut self.gold.bbq2.solo.items.clone());
        out.append(&mut self.gold.bbq2.multi.items.clone());
        out.append(&mut self.gold.bbq3.solo.items.clone());
        out.append(&mut self.gold.bbq3.multi.items.clone());
        out.append(&mut self.gold.bbq4.solo.items.clone());
        out.append(&mut self.gold.bbq4.multi.items.clone());
        out.append(&mut self.gold.bbq5.solo.items.clone());
        out.append(&mut self.gold.bbq5.multi.items.clone());
        out.append(&mut self.gold.bbq6.solo.items.clone());
        out.append(&mut self.gold.bbq6.multi.items.clone());
        out.append(&mut self.gold.bbq7.solo.items.clone());
        out.append(&mut self.gold.bbq8.multi.items.clone());
        out.append(&mut self.gold.bbq9.solo.items.clone());
        out.append(&mut self.gold.bbq9.multi.items.clone());
        out.append(&mut self.gold.bbq10.solo.items.clone());
        out.append(&mut self.gold.bbq10.multi.items.clone());
        out.append(&mut self.gold.bbq11.solo.items.clone());
        out.append(&mut self.gold.bbq11.multi.items.clone());
        out.append(&mut self.gold.bbq12.solo.items.clone());
        out.append(&mut self.gold.bbq12.multi.items.clone());
        out.append(&mut self.gold.bbq13.solo.items.clone());
        out.append(&mut self.gold.bbq13.multi.items.clone());
        out.append(&mut self.gold.bbq14.solo.items.clone());
        out.append(&mut self.gold.bbq14.multi.items.clone());
        out.append(&mut self.gold.bbq15.solo.items.clone());
        out.append(&mut self.gold.bbq15.multi.items.clone());
        out.append(&mut self.gold.bbq16.solo.items.clone());
        out.append(&mut self.gold.bbq16.multi.items.clone());
        out.append(&mut self.gold.bbq17.solo.items.clone());
        out.append(&mut self.gold.bbq17.multi.items.clone());
        out.append(&mut self.gold.bbq18.solo.items.clone());
        out.append(&mut self.gold.bbq18.multi.items.clone());
        out.append(&mut self.gold.bbq19.solo.items.clone());
        out.append(&mut self.gold.bbq19.multi.items.clone());
        out.append(&mut self.gold.bbq20.solo.items.clone());
        out.append(&mut self.gold.bbq20.multi.items.clone());
        out.append(&mut self.gold.bbq21.solo.items.clone());
        out.append(&mut self.gold.bbq21.multi.items.clone());
        out.append(&mut self.gold.bbq22.solo.items.clone());
        out.append(&mut self.gold.bbq22.multi.items.clone());
        out.append(&mut self.gold.bbq23.solo.items.clone());
        out.append(&mut self.gold.bbq23.multi.items.clone());
        out.append(&mut self.gold.bbq24.solo.items.clone());
        out.append(&mut self.gold.bbq24.multi.items.clone());
        out.append(&mut self.gold.bbq25.solo.items.clone());
        out.append(&mut self.gold.bbq25.multi.items.clone());
        out.append(&mut self.silver.bbq1.solo.items.clone());
        out.append(&mut self.silver.bbq1.multi.items.clone());
        out.append(&mut self.silver.bbq2.solo.items.clone());
        out.append(&mut self.silver.bbq2.multi.items.clone());
        out.append(&mut self.silver.bbq3.solo.items.clone());
        out.append(&mut self.silver.bbq3.multi.items.clone());
        out.append(&mut self.silver.bbq4.solo.items.clone());
        out.append(&mut self.silver.bbq4.multi.items.clone());
        out.append(&mut self.silver.bbq5.solo.items.clone());
        out.append(&mut self.silver.bbq5.multi.items.clone());
        out.append(&mut self.silver.bbq6.solo.items.clone());
        out.append(&mut self.silver.bbq6.multi.items.clone());
        out.append(&mut self.silver.bbq7.solo.items.clone());
        out.append(&mut self.silver.bbq8.multi.items.clone());
        out.append(&mut self.silver.bbq9.solo.items.clone());
        out.append(&mut self.silver.bbq9.multi.items.clone());
        out.append(&mut self.silver.bbq10.solo.items.clone());
        out.append(&mut self.silver.bbq10.multi.items.clone());
        out.append(&mut self.silver.bbq11.solo.items.clone());
        out.append(&mut self.silver.bbq11.multi.items.clone());
        out.append(&mut self.silver.bbq12.solo.items.clone());
        out.append(&mut self.silver.bbq12.multi.items.clone());
        out.append(&mut self.silver.bbq13.solo.items.clone());
        out.append(&mut self.silver.bbq13.multi.items.clone());
        out.append(&mut self.silver.bbq14.solo.items.clone());
        out.append(&mut self.silver.bbq14.multi.items.clone());
        out.append(&mut self.silver.bbq15.solo.items.clone());
        out.append(&mut self.silver.bbq15.multi.items.clone());
        out.append(&mut self.silver.bbq16.solo.items.clone());
        out.append(&mut self.silver.bbq16.multi.items.clone());
        out.append(&mut self.silver.bbq17.solo.items.clone());
        out.append(&mut self.silver.bbq17.multi.items.clone());
        out.append(&mut self.silver.bbq18.solo.items.clone());
        out.append(&mut self.silver.bbq18.multi.items.clone());
        out.append(&mut self.silver.bbq19.solo.items.clone());
        out.append(&mut self.silver.bbq19.multi.items.clone());
        out.append(&mut self.silver.bbq20.solo.items.clone());
        out.append(&mut self.silver.bbq20.multi.items.clone());
        out.append(&mut self.silver.bbq21.solo.items.clone());
        out.append(&mut self.silver.bbq21.multi.items.clone());
        out.append(&mut self.silver.bbq22.solo.items.clone());
        out.append(&mut self.silver.bbq22.multi.items.clone());
        out.append(&mut self.silver.bbq23.solo.items.clone());
        out.append(&mut self.silver.bbq23.multi.items.clone());
        out.append(&mut self.silver.bbq24.solo.items.clone());
        out.append(&mut self.silver.bbq24.multi.items.clone());
        out.append(&mut self.silver.bbq25.solo.items.clone());
        out.append(&mut self.silver.bbq25.multi.items.clone());
        out.append(&mut self.bronze.bbq1.solo.items.clone());
        out.append(&mut self.bronze.bbq1.multi.items.clone());
        out.append(&mut self.bronze.bbq2.solo.items.clone());
        out.append(&mut self.bronze.bbq2.multi.items.clone());
        out.append(&mut self.bronze.bbq3.solo.items.clone());
        out.append(&mut self.bronze.bbq3.multi.items.clone());
        out.append(&mut self.bronze.bbq4.solo.items.clone());
        out.append(&mut self.bronze.bbq4.multi.items.clone());
        out.append(&mut self.bronze.bbq5.solo.items.clone());
        out.append(&mut self.bronze.bbq5.multi.items.clone());
        out.append(&mut self.bronze.bbq6.solo.items.clone());
        out.append(&mut self.bronze.bbq6.multi.items.clone());
        out.append(&mut self.bronze.bbq7.solo.items.clone());
        out.append(&mut self.bronze.bbq8.multi.items.clone());
        out.append(&mut self.bronze.bbq9.solo.items.clone());
        out.append(&mut self.bronze.bbq9.multi.items.clone());
        out.append(&mut self.bronze.bbq10.solo.items.clone());
        out.append(&mut self.bronze.bbq10.multi.items.clone());
        out.append(&mut self.bronze.bbq11.solo.items.clone());
        out.append(&mut self.bronze.bbq11.multi.items.clone());
        out.append(&mut self.bronze.bbq12.solo.items.clone());
        out.append(&mut self.bronze.bbq12.multi.items.clone());
        out.append(&mut self.bronze.bbq13.solo.items.clone());
        out.append(&mut self.bronze.bbq13.multi.items.clone());
        out.append(&mut self.bronze.bbq14.solo.items.clone());
        out.append(&mut self.bronze.bbq14.multi.items.clone());
        out.append(&mut self.bronze.bbq15.solo.items.clone());
        out.append(&mut self.bronze.bbq15.multi.items.clone());
        out.append(&mut self.bronze.bbq16.solo.items.clone());
        out.append(&mut self.bronze.bbq16.multi.items.clone());
        out.append(&mut self.bronze.bbq17.solo.items.clone());
        out.append(&mut self.bronze.bbq17.multi.items.clone());
        out.append(&mut self.bronze.bbq18.solo.items.clone());
        out.append(&mut self.bronze.bbq18.multi.items.clone());
        out.append(&mut self.bronze.bbq19.solo.items.clone());
        out.append(&mut self.bronze.bbq19.multi.items.clone());
        out.append(&mut self.bronze.bbq20.solo.items.clone());
        out.append(&mut self.bronze.bbq20.multi.items.clone());
        out.append(&mut self.bronze.bbq21.solo.items.clone());
        out.append(&mut self.bronze.bbq21.multi.items.clone());
        out.append(&mut self.bronze.bbq22.solo.items.clone());
        out.append(&mut self.bronze.bbq22.multi.items.clone());
        out.append(&mut self.bronze.bbq23.solo.items.clone());
        out.append(&mut self.bronze.bbq23.multi.items.clone());
        out.append(&mut self.bronze.bbq24.solo.items.clone());
        out.append(&mut self.bronze.bbq24.multi.items.clone());
        out.append(&mut self.bronze.bbq25.solo.items.clone());
        out.append(&mut self.bronze.bbq25.multi.items.clone());
        out.append(&mut self.free.bbq1.solo.items.clone());
        out.append(&mut self.free.bbq1.multi.items.clone());
        out.append(&mut self.free.bbq2.solo.items.clone());
        out.append(&mut self.free.bbq2.multi.items.clone());
        out.append(&mut self.free.bbq3.solo.items.clone());
        out.append(&mut self.free.bbq3.multi.items.clone());
        out.append(&mut self.free.bbq4.solo.items.clone());
        out.append(&mut self.free.bbq4.multi.items.clone());
        out.append(&mut self.free.bbq5.solo.items.clone());
        out.append(&mut self.free.bbq5.multi.items.clone());
        out.append(&mut self.free.bbq6.solo.items.clone());
        out.append(&mut self.free.bbq6.multi.items.clone());
        out.append(&mut self.free.bbq7.solo.items.clone());
        out.append(&mut self.free.bbq8.multi.items.clone());
        out.append(&mut self.free.bbq9.solo.items.clone());
        out.append(&mut self.free.bbq9.multi.items.clone());
        out.append(&mut self.free.bbq10.solo.items.clone());
        out.append(&mut self.free.bbq10.multi.items.clone());
        out.append(&mut self.free.bbq11.solo.items.clone());
        out.append(&mut self.free.bbq11.multi.items.clone());
        out.append(&mut self.free.bbq12.solo.items.clone());
        out.append(&mut self.free.bbq12.multi.items.clone());
        out.append(&mut self.free.bbq13.solo.items.clone());
        out.append(&mut self.free.bbq13.multi.items.clone());
        out.append(&mut self.free.bbq14.solo.items.clone());
        out.append(&mut self.free.bbq14.multi.items.clone());
        out.append(&mut self.free.bbq15.solo.items.clone());
        out.append(&mut self.free.bbq15.multi.items.clone());
        out.append(&mut self.free.bbq16.solo.items.clone());
        out.append(&mut self.free.bbq16.multi.items.clone());
        out.append(&mut self.free.bbq17.solo.items.clone());
        out.append(&mut self.free.bbq17.multi.items.clone());
        out.append(&mut self.free.bbq18.solo.items.clone());
        out.append(&mut self.free.bbq18.multi.items.clone());
        out.append(&mut self.free.bbq19.solo.items.clone());
        out.append(&mut self.free.bbq19.multi.items.clone());
        out.append(&mut self.free.bbq20.solo.items.clone());
        out.append(&mut self.free.bbq20.multi.items.clone());
        out.append(&mut self.free.bbq21.solo.items.clone());
        out.append(&mut self.free.bbq21.multi.items.clone());
        out.append(&mut self.free.bbq22.solo.items.clone());
        out.append(&mut self.free.bbq22.multi.items.clone());
        out.append(&mut self.free.bbq23.solo.items.clone());
        out.append(&mut self.free.bbq23.multi.items.clone());
        out.append(&mut self.free.bbq24.solo.items.clone());
        out.append(&mut self.free.bbq24.multi.items.clone());
        out.append(&mut self.free.bbq25.solo.items.clone());
        out.append(&mut self.free.bbq25.multi.items.clone());
        out.append(&mut self.event.bbq1.solo.items.clone());
        out.append(&mut self.event.bbq1.multi.items.clone());
        out.append(&mut self.event.bbq2.solo.items.clone());
        out.append(&mut self.event.bbq2.multi.items.clone());
        out.append(&mut self.event.bbq3.solo.items.clone());
        out.append(&mut self.event.bbq3.multi.items.clone());
        out.append(&mut self.event.bbq4.solo.items.clone());
        out.append(&mut self.event.bbq4.multi.items.clone());
        out.append(&mut self.event.bbq5.solo.items.clone());
        out.append(&mut self.event.bbq5.multi.items.clone());
        out.append(&mut self.event.bbq6.solo.items.clone());
        out.append(&mut self.event.bbq6.multi.items.clone());
        out.append(&mut self.event.bbq7.solo.items.clone());
        out.append(&mut self.event.bbq8.multi.items.clone());
        out.append(&mut self.event.bbq9.solo.items.clone());
        out.append(&mut self.event.bbq9.multi.items.clone());
        out.append(&mut self.event.bbq10.solo.items.clone());
        out.append(&mut self.event.bbq10.multi.items.clone());
        out.append(&mut self.event.bbq11.solo.items.clone());
        out.append(&mut self.event.bbq11.multi.items.clone());
        out.append(&mut self.event.bbq12.solo.items.clone());
        out.append(&mut self.event.bbq12.multi.items.clone());
        out.append(&mut self.event.bbq13.solo.items.clone());
        out.append(&mut self.event.bbq13.multi.items.clone());
        out.append(&mut self.event.bbq14.solo.items.clone());
        out.append(&mut self.event.bbq14.multi.items.clone());
        out.append(&mut self.event.bbq15.solo.items.clone());
        out.append(&mut self.event.bbq15.multi.items.clone());
        out.append(&mut self.event.bbq16.solo.items.clone());
        out.append(&mut self.event.bbq16.multi.items.clone());
        out.append(&mut self.event.bbq17.solo.items.clone());
        out.append(&mut self.event.bbq17.multi.items.clone());
        out.append(&mut self.event.bbq18.solo.items.clone());
        out.append(&mut self.event.bbq18.multi.items.clone());
        out.append(&mut self.event.bbq19.solo.items.clone());
        out.append(&mut self.event.bbq19.multi.items.clone());
        out.append(&mut self.event.bbq20.solo.items.clone());
        out.append(&mut self.event.bbq20.multi.items.clone());
        out.append(&mut self.event.bbq21.solo.items.clone());
        out.append(&mut self.event.bbq21.multi.items.clone());
        out.append(&mut self.event.bbq22.solo.items.clone());
        out.append(&mut self.event.bbq22.multi.items.clone());
        out.append(&mut self.event.bbq23.solo.items.clone());
        out.append(&mut self.event.bbq23.multi.items.clone());
        out.append(&mut self.event.bbq24.solo.items.clone());
        out.append(&mut self.event.bbq24.multi.items.clone());
        out.append(&mut self.event.bbq25.solo.items.clone());
        out.append(&mut self.event.bbq25.multi.items.clone());
        out
    }
    pub async fn new()->Result<Self,MyErr>{
        let file = tokio::fs::read_to_string(Bounty::path()).await?;
        Ok(serde_json::from_str(&file)?)
    }
    pub async fn save(&self)->Result<(),Error>{
        let string = serde_json::to_string_pretty(&self)?;
        tokio::fs::write(Bounty::path(), string.as_bytes()).await?;
        Ok(())
    }
    pub fn refresh(&mut self,reff:&BountyRefresh){
        self.free.bbq1.cooldown.replace(reff.bbq1);
        self.free.bbq2.cooldown.replace(reff.bbq2);
        self.free.bbq3.cooldown.replace(reff.bbq3);
        self.free.bbq4.cooldown.replace(reff.bbq4);
        self.free.bbq5.cooldown.replace(reff.bbq5);
        self.free.bbq6.cooldown.replace(reff.bbq6);
        self.free.bbq7.cooldown.replace(reff.bbq7);
        self.free.bbq8.cooldown.replace(reff.bbq8);
        self.free.bbq9.cooldown.replace(reff.bbq9);
        self.free.bbq10.cooldown.replace(reff.bbq10);
        self.free.bbq11.cooldown.replace(reff.bbq11);
        self.free.bbq12.cooldown.replace(reff.bbq12);
        self.free.bbq13.cooldown.replace(reff.bbq13);
        self.free.bbq14.cooldown.replace(reff.bbq14);
        self.free.bbq15.cooldown.replace(reff.bbq15);
        self.free.bbq16.cooldown.replace(reff.bbq16);
        self.free.bbq17.cooldown.replace(reff.bbq17);
        self.free.bbq18.cooldown.replace(reff.bbq18);
        self.free.bbq19.cooldown.replace(reff.bbq19);
        self.free.bbq20.cooldown.replace(reff.bbq20);
        self.free.bbq21.cooldown.replace(reff.bbq21);
        self.free.bbq22.cooldown.replace(reff.bbq22);
        self.free.bbq23.cooldown.replace(reff.bbq23);
        self.free.bbq24.cooldown.replace(reff.bbq24);
        self.free.bbq25.cooldown.replace(reff.bbq25);
    }
    pub async fn cooldown(&self,bnd:&SlashBundle<'_>)->Result<(),MyErr>{
        let mut msg = ChannelId::new(bnd.init.bounty.cooldown_ch)
            .message(&bnd.ctx.http, MessageId::new(bnd.init.bounty.cooldown_msg)).await?;
        msg.edit(&bnd.ctx.http, EditMessage::new().embed(self.free.cooldown_embed())).await?;
        Ok(())
    }
    pub fn set_cd(&mut self,bbq:&BBQ,cd:u32){
        let x = bbq.get_bounty(&self.free);
        x.cooldown.replace(cd);
    }
}
impl BountyBBQ {
    pub fn cooldown_embed(&self)->CreateEmbed{
        let mut idk = format!("```\nBBQ01 ====== {} left",self.bbq1.cooldown.borrow());
        idk.push_str(&format!("\nBBQ02 ====== {} left",self.bbq2.cooldown.borrow()));
        idk.push_str(&format!("\nBBQ03 ====== {} left",self.bbq3.cooldown.borrow()));
        idk.push_str(&format!("\nBBQ04 ====== {} left",self.bbq4.cooldown.borrow()));
        idk.push_str(&format!("\nBBQ05 ====== {} left",self.bbq5.cooldown.borrow()));
        idk.push_str(&format!("\nBBQ06 ====== {} left",self.bbq6.cooldown.borrow()));
        idk.push_str(&format!("\nBBQ07 ====== {} left",self.bbq7.cooldown.borrow()));
        idk.push_str(&format!("\nBBQ08 ====== {} left",self.bbq8.cooldown.borrow()));
        idk.push_str(&format!("\nBBQ09 ====== {} left",self.bbq9.cooldown.borrow()));
        idk.push_str(&format!("\nBBQ10 ====== {} left",self.bbq10.cooldown.borrow()));
        idk.push_str(&format!("\nBBQ11 ====== {} left",self.bbq11.cooldown.borrow()));
        idk.push_str(&format!("\nBBQ12 ====== {} left",self.bbq12.cooldown.borrow()));
        idk.push_str(&format!("\nBBQ13 ====== {} left",self.bbq13.cooldown.borrow()));
        idk.push_str(&format!("\nBBQ14 ====== {} left",self.bbq14.cooldown.borrow()));
        idk.push_str(&format!("\nBBQ15 ====== {} left",self.bbq15.cooldown.borrow()));
        idk.push_str(&format!("\nBBQ16 ====== {} left",self.bbq16.cooldown.borrow()));
        idk.push_str(&format!("\nBBQ17 ====== {} left",self.bbq17.cooldown.borrow()));
        idk.push_str(&format!("\nBBQ18 ====== {} left",self.bbq18.cooldown.borrow()));
        idk.push_str(&format!("\nBBQ19 ====== {} left",self.bbq19.cooldown.borrow()));
        idk.push_str(&format!("\nBBQ20 ====== {} left",self.bbq20.cooldown.borrow()));
        idk.push_str(&format!("\nBBQ21 ====== {} left",self.bbq21.cooldown.borrow()));
        idk.push_str(&format!("\nBBQ22 ====== {} left",self.bbq22.cooldown.borrow()));
        idk.push_str(&format!("\nBBQ23 ====== {} left",self.bbq23.cooldown.borrow()));
        idk.push_str(&format!("\nBBQ24 ====== {} left",self.bbq24.cooldown.borrow()));
        idk.push_str(&format!("\nBBQ25 ====== {} left\n```",self.bbq25.cooldown.borrow()));
        CreateEmbed::new().title("Free Category Cooldown").description(idk)
    }
}
impl Default for BountyDesc{
    fn default() -> Self {
        BountyDesc { description: "this is bbq description".to_owned(), cooldown: RefCell::new(0), solo:BountyReward::default(),multi:BountyReward::default(),
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
            .image("attachment://title.jpg").author(CreateEmbedAuthor::new(&user.name).icon_url(user.face()))
    }
    pub async fn send_title(&self,bnd:&SlashBundle<'_>,user:&User)->Result<(),MyErr>{
        let image = self.image.title(&user.static_avatar_url().unwrap_or(user.default_avatar_url())).await?;
        ChannelId::new(bnd.init.bounty.promotion_ch)
            .send_message(&bnd.ctx.http, CreateMessage::new().add_file(CreateAttachment::bytes(image, "title.jpg"))
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
    pub async fn check(data:&str)->Result<(),MyErr>{
        let x = serde_json::from_str::<Self>(data)?;
        for i in &x.custom{
            if i.db_code != 0{
                return Err(MyErr::Custom("you cant set value other than 0 for custom title's db_code".to_owned()));
            }
        }
        x.save().await?;
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
    fn name(code:&str)->String{
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
    #[ignore = "already had"]
    async fn get_json() {
        let x = Bounty::default();
        let y = BountyRefresh::default();
        let z = BountyTitle::default();
        x.save().await.unwrap();
        y.save().await.unwrap();
        z.save().await.unwrap();
    }
    #[tokio::test]
    #[ignore = "already tested"]
    async fn refresh() {
        let mut x = Bounty::default();
        let y = BountyRefresh::new().await.unwrap();
        x.refresh(&y);
        x.save().await.unwrap();
    }
    #[tokio::test]
    #[ignore = "already tested"]
    async fn reset_cd() {
        let mut x = Bounty::new().await.unwrap();
        x.set_cd(&BBQ::BBQ01, 100);
        assert_eq!(x.free.bbq1.cooldown.borrow().to_owned(),100);
        x.save().await.unwrap();
    }
}
