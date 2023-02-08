use serenity::all::*;
use crate::material::ItemList;
use serde_json::Value;
use crate::{Init,ErrorLog};

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
fn compute(key:u8,val:&str)->Option<Vec<AutocompleteChoice>>{
    if val == ""{
        return None;
    }
    let item = ItemList::new(key)?;
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
    fn predict(&self)->Option<Vec<AutocompleteChoice>>{
        match self{
            SubCommand::Leg(val)=>compute(0, val),
            SubCommand::Head(val)=>compute(1, val),
            SubCommand::Chest(val)=>compute(2, val),
            SubCommand::Arms(val)=>compute(3, val),
            SubCommand::Waist(val)=>compute(4, val),
            SubCommand::Melee(val)=>compute(5, val),
            SubCommand::Ranged(val)=>compute(6, val),
            SubCommand::Item(val)=>compute(7, val)
        }
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
