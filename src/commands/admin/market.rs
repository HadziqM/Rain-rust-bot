use serenity::all::*;
use crate::{material::ItemList, reusable::bitwise::ItemCode};
use serde_json::Value;
use crate::{Init,ErrorLog,Register,Components};

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
    fn new(cmd:&CommandInteraction)->Option<MarketHandle>{
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
                            if x > &65025{
                                return None;
                            }
                            count = *x as u16;
                        }
                        CommandDataOptionValue::Number(x)=>{price = Some(*x as i32)}
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
                return Some(MarketHandle{data,user,count,price});
            }
        }
        None
    }
    fn code(&self)->ItemCode{
        ItemCode { key: self.data.value().to_owned(), count:self.count, types: self.data.code() }
    }
}
impl SubCommand{
    fn new(cmd:&CommandInteraction)->Option<SubCommand>{
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
                        return Some(data);
                    }
                }
            }
        }
        None
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
    fn predict(&self)->Option<Vec<AutocompleteChoice>>{
        let val = self.value();
        if val == ""{
            return None;
        }
        let item = ItemList::new(self.code())?;
        let out = item.value().iter().filter(|(_,f)|f.to_lowercase()
            .starts_with(&val.to_lowercase()))
            .map(|(&k,&v)|AutocompleteChoice{value:Value::String(k.to_owned())
            ,name:v.to_owned()}).collect::<Vec<_>>();
        if out.len() == 0{
            return None;
        }else if out.len()>15{
            return Some(out[0..15].to_owned());
        }
        Some(out)
    }
}
pub async fn run_autocomplete(ctx:&Context,cmd:&CommandInteraction,init:&Init){
    let mut err = ErrorLog::new(ctx, init, &cmd.user).await;
    let option = match SubCommand::new(cmd){
        Some(x)=>x,
        None=>{return ;}
    };
    let choice = match option.predict(){
        Some(x)=>x,
        None=>{return;}
    };
    if let Err(why) = cmd.create_response(&ctx.http, CreateInteractionResponse::Autocomplete(
        CreateAutocompleteResponse::new().set_choices(choice))).await{
        err.discord_error(why.to_string(),"item autocomplete response").await;
    }
}
pub async fn run(ctx:&Context,cmd:&CommandInteraction,init:&Init){
    let mut err = ErrorLog::new(ctx, init, &cmd.user).await;
    let option = match MarketHandle::new(cmd){
        Some(x)=>x,
        None=>{
            err.change_error("invalid input".to_string(), "market command", "check your input especially the count item".to_string());
            return err.log_slash(cmd, false).await;
        }
    };
    let user = option.user.to_user(&ctx.http).await.unwrap();
    let mut reg =match Register::default_user(ctx, cmd, init, "market command",&user).await{
        Some(x)=>x,
        None=>{return;}
    };
    match reg.pg.market(&option.code(), reg.cid, option.price).await{
        Ok(_)=>{
            if let Err(why)=cmd.create_response(&ctx.http, Components::interaction_response("sended distribution data", true)).await{
                err.discord_error(why.to_string(), "letting know market is done").await;
            }
        }
        Err(why)=>{
            err.pgcon_error(why.to_string(), "sending data", cmd).await;
        }
    }
    reg.pg.close().await;
}