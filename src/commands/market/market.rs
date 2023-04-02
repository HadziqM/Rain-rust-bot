use std::time::Duration;

use crate::{MyErr,ItemPedia,Components,SlashBundle,Reg, reusable::{bitwise::ItemCode, component::{Mytrait, bounty::Title}}, event::interaction::ComponentBundle};
use serenity::{all::*, futures::StreamExt};
use crate::reusable::component::market::{Market,Item,Trading};


struct Handle{
    market:Market,
    item:Item,
    bought:u16,
}
impl Handle{
    async fn new(cmd:&CommandInteraction)->Result<Handle,MyErr>{
        let market = Market::new().await?;
        let mut bought = 1;
        let mut item = Item::default();
        for pat in &cmd.data.options {
            match &pat.value{
                CommandDataOptionValue::SubCommand(x)=>{
                    for sub in x{
                        match &sub.value{
                        CommandDataOptionValue::Integer(x)=>{
                            bought = match u16::try_from(x.to_owned()) {
                                Ok(y) => {
                                    if y==0{
                                        return Err(MyErr::Custom(format!("you got number wrong, only allowed between 1- 2^16 && <item stock>, then you input 0")));
                                    }
                                    y
                                },
                                Err(why) =>{
                                    return Err(MyErr::Custom(format!("you got number wrong, only allowed between 1- 2^16 && <item stock>, then you input {x}\nthe error code :\n{why:?}")));
                                }
                            }
                        }
                        CommandDataOptionValue::String(x)=>{
                            item = match market.item_hash().get(x){
                                Some(y)=>y.to_owned(),
                                None=>{
                                    return Err(MyErr::Custom(format!("you have input command with \"{x}\" and our given choice didnt use or have that, use this command again to try again but make sure you properly select the option available")));
                                }
                            }
                        }
                        _=>{continue;}
                        }
                    }
                }
                _=>{continue;}
            }
        }
        Ok(Handle{market,bought,item})
    }
    async fn check(&self,reg:&Reg<'_>,pedia:&ItemPedia)->Result<Bought,MyErr>{
        let (bc,event) = tokio::join!(reg.pg.get_coin(),reg.pg.get_event());
        let (bc,event) = (bc?,event?);
        let title = Title::new(event.title as u8).discount();
        let bon = (1.0-title)*100.0;
        let disc = format!("{}%",bon as u32);
        let total = self.item.price as i64 * self.bought as i64;
        let discounted = (total as f32 * title) as i64;
        let item  = self.item.item.count as i64 * self.bought as i64;
        if self.bought as u32 > self.item.treshold{
            return Err(MyErr::Custom(format!("you bought {} item, but market now only had {} item",self.bought,self.item.treshold)));
        }
        if bc < discounted as i32{
            return Err(MyErr::Custom(format!("you only have {bc} Bounty Coin but you need {total} for transaction, collect more BC for succesfull transaction")));
        }
        if total > 2_147_483_647{
            return Err(MyErr::Custom(format!("you try to begin transaction with {total} BC, the money amount allowed by our goverment is not higher than 2.147.483.647 BC per transaction or you get suspected as terrorist, try lower your item bought")));
        }
        if item > 65_535{
            return Err(MyErr::Custom(format!("you try to bought {item} total items (bought times item bundle value), while our courier can only carry 65.535 items at their prime, try reduce your bought item")));
        }
        let change = bc as i64 - discounted;
        Ok(Bought 
           { item: self.item.item.text(pedia).unwrap(), qty: self.bought as i64,
           total, change, former:bc, price:self.item.price as i32, unit:"item(s)".to_string(),discounted,disc})
    }
    async fn transaction(&mut self,reg:&Reg<'_>,pedia:&ItemPedia)->Result<(),MyErr>{
        let event = reg.pg.get_event().await?;
        let price = ((self.item.price * self.bought as u32) as f32 * Title::new(event.title as u8).discount()) as u32;
        let count  = self.item.item.count * self.bought;
        self.item.treshold -= self.bought as u32;
        self.market.bought(&self.item);
        let item = ItemCode{key:self.item.item.key.to_owned(),types:self.item.item.types,count};
        reg.pg.market_user(&item, reg.cid, price, pedia).await?;
        Ok(())
    }
    async fn post_transaction(&self,bnd:&SlashBundle<'_>)->Result<(),MyErr>{
        self.market.save().await?;
        self.market.update(bnd.ctx, bnd.init, bnd.pedia).await?;
        Ok(())
    }
}
pub struct Bought{
    item:String,
    qty:i64,
    total:i64,
    change:i64,
    former:i32,
    price:i32,
    discounted:i64,
    disc:String,
    unit:String
}
impl Bought{
    pub fn new(item:String,qty:i64,total:i64,change:i64,former:i32,price:i32,unit:String,discounted:i64,disc:String)->Self{
        Bought { item, qty, total, change, former, price, unit, discounted,disc }
    }
    pub fn create_embed(&self,bnd:&SlashBundle<'_>)->CreateEmbed{
        CreateEmbed::new().title("Receipt").author(CreateEmbedAuthor::new(&bnd.cmd.user.name).icon_url(bnd.cmd.user.face()))
            .description(format!("{} Receipt at <t:{}:F>",&bnd.cmd.user,crate::reusable::utils::MyTime::now()))
            .field("Item", format!("{}\nBought x{} {} ",&self.item,self.qty,&self.unit), false)
            .field("Price", format!("Per {}: {} x {} = {}",&self.unit
                ,Market::currency(self.price as i64),self.qty,Market::currency(self.total as i64)), false)
            .field("Discounted", format!("Discount {} for {} = {}",&self.disc
                ,Market::currency(self.total),Market::currency(self.discounted)), false)
            .field("Currency", format!("Your Coin = {} - {} = {}",Market::currency(self.former as i64),Market::currency(self.discounted as i64),Market::currency(self.change as i64)), false)
            .color(crate::reusable::utils::Color::Random.throw())
    }
    async fn wtf(bnd:&SlashBundle<'_>,x:&ComponentInteraction)->Result<bool,MyErr>{
        if x.user != bnd.cmd.user{
            x.response(bnd.ctx, Components::interaction_response("the button is not for you", true)).await?;
            return Ok(false);
        }
        Ok(true)
    }
    pub async fn confirmation(&self,bnd:&SlashBundle<'_>)->Result<bool,MyErr>{
        let arow = CreateActionRow::Buttons(vec![
            Components::normal_button("Confirm", "yes", ButtonStyle::Primary, "👌"),
            Components::normal_button("Cencel", "nope", ButtonStyle::Danger, "👎")
        ]);
        Components::response_adv(bnd, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
            .embed(self.create_embed(bnd)).components(vec![arow]))).await?;
        let mut msg = bnd.cmd.get_msg(bnd.ctx).await?;
        let mut reply = msg.await_component_interactions(bnd.ctx).timeout(Duration::new(60, 0)).stream();
        while let Some(x) = reply.next().await{
            let bnd2 = ComponentBundle{cmd:&x,init:bnd.init,ctx:bnd.ctx,pedia:bnd.pedia,image:bnd.image};
            match x.data.custom_id.as_str(){
                "yes" => {
                    if Bought::wtf(bnd, &x).await?{
                        Components::response(&bnd2, "bought", true).await?;
                        msg.edit(&bnd.ctx.http, EditMessage::new().components(Vec::new())).await?;
                        return Ok(true);
                    }
                    continue;
                }
                "nope" => {
                    if Bought::wtf(bnd, &x).await?{
                        Components::response(&bnd2, "cenceled", true).await?;
                        msg.delete(&bnd.ctx.http).await?;
                        return Ok(false);
                    }
                    continue;
                }
                _ => {continue;}
            }
        }
        Ok(false)
    }
}
pub async fn auto(bnd:&SlashBundle<'_>,focus:&str)->Result<(),MyErr>{
    let market = Market::new().await?;
    bnd.cmd.create_response(&bnd.ctx.http, CreateInteractionResponse::Autocomplete(CreateAutocompleteResponse::new()
        .set_choices(market.auto(bnd.pedia, focus)))).await?;
    Ok(())
}

pub async fn slash(bnd:&SlashBundle<'_>,reg:&Reg<'_>)->Result<(),MyErr>{
    let trade = Trading::new().await?;
    if !trade.market.enabled{
        return Err(MyErr::Custom("tradimng market is currently disabled".to_string()));
    }
    let mut handle = Handle::new(bnd.cmd).await?;
    let receipt = handle.check(reg, bnd.pedia).await?;
    if receipt.confirmation(bnd).await?{
        handle.transaction(reg, bnd.pedia).await?;
    }
    Ok(())
}
