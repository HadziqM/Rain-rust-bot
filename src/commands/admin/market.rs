use serenity::all::*;
use crate::reusable::bitwise::ItemCode;
use serde_json::Value;
use crate::{Reg,Components,ItemPedia,SlashBundle,MyErr};

enum SubCommand{
    Item(String),
    Head(String),
    Arms(String),
    Waist(String),
    Chest(String),
    Leg(String),
    Melee(String),
    Ranged(String)
}
struct MarketHandle{
    data:SubCommand,
    user:UserId,
    count:u16,
    price:Option<i32>
}
impl MarketHandle {
    fn new(cmd:&CommandInteraction)->Result<MarketHandle,MyErr>{
        let err = MyErr::Custom("invalid input, please check your input especially on unit".to_string());
        for sub in &cmd.data.options{
            if let CommandDataOptionValue::SubCommand(cmx) = &sub.value{
                let mut user = UserId::default();
                let mut count = 1;
                let mut price = None;
                let mut data = SubCommand::Item("0000".to_owned());
                for opt in cmx{
                    match &opt.value{
                        CommandDataOptionValue::User(x)=>{user=*x}
                        CommandDataOptionValue::Integer(x)=>{
                            if &opt.name == "price"{
                                price  = match i32::try_from(*x){
                                    Ok(y)=>Some(y),
                                    Err(_)=>{return Err(err);}
                                }
                            }else{
                                count = match u16::try_from(*x){
                                    Ok(y)=>y,
                                    Err(_)=>{return Err(err);}
                                };
                                if count == 0 {
                                    return Err(err);
                                }
                            }
                        }
                        CommandDataOptionValue::String(value)=>{
                            let value = value.to_owned();
                            data = match sub.name.as_str(){
                                "item"=>SubCommand::Item(value),
                                "head"=>SubCommand::Head(value),
                                "arms"=>SubCommand::Arms(value),
                                "waist"=>SubCommand::Waist(value),
                                "chest"=>SubCommand::Chest(value),
                                "leg"=>SubCommand::Leg(value),
                                "melee"=>SubCommand::Melee(value),
                                "ranged"=>SubCommand::Ranged(value),
                                _=>{continue;}
                            }
                        }
                        _=>{continue;}
                    }
                }
                return Ok(MarketHandle{data,user,count,price});
            }
        }
        Err(err)
    }
    fn code(&self)->ItemCode{
        ItemCode { key: self.data.value().to_owned(), count:self.count, types: self.data.code() }
    }
}
impl SubCommand{
    fn new(cmd:&CommandInteraction)->Result<SubCommand,MyErr>{
        for sub in &cmd.data.options{
            if let CommandDataOptionValue::SubCommand(cmx) = &sub.value{
                for opt in cmx{
                    if let CommandDataOptionValue::Autocomplete {kind:_,value} = &opt.value{
                        let value = value.to_owned();
                        let data = match sub.name.as_str(){
                            "item"=>SubCommand::Item(value),
                            "head"=>SubCommand::Head(value),
                            "arms"=>SubCommand::Arms(value),
                            "waist"=>SubCommand::Waist(value),
                            "chest"=>SubCommand::Chest(value),
                            "leg"=>SubCommand::Leg(value),
                            "melee"=>SubCommand::Melee(value),
                            "ranged"=>SubCommand::Ranged(value),
                            _=>{continue;}
                        };
                        return Ok(data);
                    }
                }
            }
        }
        Err(MyErr::Custom("cant get the data option".to_string()))
    }
    fn code(&self)->u8{
        match self{
            SubCommand::Leg(_val)=>0,
            SubCommand::Head(_val)=>1,
            SubCommand::Chest(_val)=>2,
            SubCommand::Arms(_val)=>3,
            SubCommand::Waist(_val)=>4,
            SubCommand::Melee(_val)=>5,
            SubCommand::Ranged(_val)=>6,
            SubCommand::Item(_)=>7
        }
    }
    fn value(&self)->&String{
        match self{
            SubCommand::Leg(val)=>val,
            SubCommand::Head(val)=>val,
            SubCommand::Chest(val)=>val,
            SubCommand::Arms(val)=>val,
            SubCommand::Waist(val)=>val,
            SubCommand::Melee(val)=>val,
            SubCommand::Ranged(val)=>val,
            SubCommand::Item(val)=>val
        }
    }
    fn predict(&self,pedia:&ItemPedia)->Result<Vec<AutocompleteChoice>,MyErr>{
        let val = self.value();
        let item = pedia.types.get(&self.code()).unwrap();
        let out = item.iter().filter(|(k,f)|{
                let flat_val = val.to_lowercase();
                let flat_tar = f.to_lowercase();
                if flat_tar.starts_with(&flat_val)||
                    flat_tar.contains(&flat_val)||
                    k.starts_with(&val.to_uppercase())||
                    k.contains(&val.to_uppercase()){
                    return true;
            }
             false
            }
            )
            .map(|(&k,&v)|{
                let mut name = format!("{} - {}",k,v);
                if v.len() > 100{
                    name = name[0..90].to_owned();
                }
                AutocompleteChoice{value:Value::String(k.to_owned()),name}
            }).collect::<Vec<_>>();
        if out.len() == 0{
            return Err(MyErr::Custom("no result match for your input".to_string()));
        }else if out.len()>15{
            return Ok(out[0..15].to_owned());
        }
        Ok(out)
    }
}
pub async fn auto(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    let option = SubCommand::new(bnd.cmd)?;
    let choice = option.predict(bnd.pedia)?;
    let content = CreateInteractionResponse::Autocomplete(CreateAutocompleteResponse::new().set_choices(choice));
    Components::response_adv(bnd, content).await?;
    Ok(())
}
pub async fn slash(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    let option = MarketHandle::new(bnd.cmd)?;
    let user = option.user.to_user(&bnd.ctx.http).await.unwrap();
    let mut reg = Reg::check(bnd, &user).await?;
    reg.pg.market(&option.code(), reg.cid, option.price,bnd.pedia).await?;
    Components::response(bnd, "sended distribution data", true).await?;
    reg.pg.close().await;
    Ok(())
}
