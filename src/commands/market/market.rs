use crate::{MyErr,ItemPedia,Components,SlashBundle,Reg,Mybundle,Mytrait, reusable::bitwise::ItemCode};
use serenity::all::*;
use crate::reusable::component::market::{Market,Item};
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
        Ok(Handle{market,bought,item})
    }
    async fn check(&self,reg:&Reg<'_>,pedia:&ItemPedia)->Result<Bought,MyErr>{
        let bc = reg.pg.get_coin().await?;
        let total = self.item.price as u64 * self.bought as u64;
        let item  = self.item.item.count as u32 * self.bought as u32;
        if self.bought as u32 > self.item.treshold{
            return Err(MyErr::Custom(format!("you bought {} item, but market now only had {} item",self.bought,self.item.treshold)));
        }
        if bc < total as i32{
            return Err(MyErr::Custom(format!("you only have {bc} Bounty Coin but you need {total} for transaction, collect more BC for succesfull transaction")));
        }
        if total > 2_147_483_647{
            return Err(MyErr::Custom(format!("you try to begin transaction with {total} BC, the money amount allowed by our goverment is not higher than 2.147.483.647 BC per transaction or you get suspected as terrorist, try lower your item bought")));
        }
        if item > 65_535{
            return Err(MyErr::Custom(format!("you try to bought {item} total items (bought times item bundle value), while our courier can only carry 65.535 items at their prime, try reduce your bought item")));
        }
        let change = bc - total as i32;
        Ok(Bought { item: self.item.item.text(pedia).unwrap(), qty: self.bought, total, change, former:bc, price:self.item.price })
    }
    async fn transaction(&mut self,reg:&Reg<'_>,pedia:&ItemPedia)->Result<(),MyErr>{
        let price = self.item.price * self.bought as u32;
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
struct Bought{
    item:String,
    qty:u16,
    total:u64,
    change:i32,
    former:i32,
    price:u32
}
impl Bought {
    fn create_embed(&self,bnd:&SlashBundle<'_>)->CreateEmbed{
        CreateEmbed::new().title("Receipt").author(CreateEmbedAuthor::new(&bnd.cmd.user.name).icon_url(bnd.cmd.user.face()))
            .description(format!("{} Receipt at <t:{}:F>",&bnd.cmd.user,crate::reusable::utils::MyTime::now()))
            .field("Item", format!("{}\nBought x{} times ",&self.item,self.qty), false)
            .field("Price", format!("Per Item: {} x {} = {}",currency(self.price as u64),self.qty,currency(self.total)), false)
            .field("Currency", format!("Your Coin = {} - {} = {}",currency(self.former as u64),currency(self.total),currency(self.change as u64)), false)
            .color(crate::reusable::utils::Color::Random.throw())
    }
}
fn currency(cur:u64)->String{
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
#[hertz::hertz_auto]
async fn idk(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    for pat in &bnd.cmd.data.options {
        if let CommandDataOptionValue::Autocomplete { kind:_, value } = &pat.value{
            let market = Market::new().await?;
            bnd.cmd.create_response(&bnd.ctx.http, CreateInteractionResponse::Autocomplete(CreateAutocompleteResponse::new()
                .set_choices(market.auto(bnd.pedia, &value)))).await?;
        }
    }
    Ok(())
}

#[hertz::hertz_slash_reg(60,false)]
async fn slash(bnd:&SlashBundle<'_>,mut reg:Reg<'_>)->Result<(),MyErr>{
    let mut handle = Handle::new(bnd.cmd).await?;
    let receipt = handle.check(&reg,bnd.pedia).await?;
    handle.transaction(&reg, bnd.pedia).await?;
    Components::response_adv(bnd,CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().embed(receipt.create_embed(bnd)))).await?;
    handle.post_transaction(bnd).await?;
    reg.pg.close().await;
    Ok(())
}
