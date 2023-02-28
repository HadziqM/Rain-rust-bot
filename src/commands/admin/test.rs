use crate::{MyErr,SlashBundle,Mytrait,Mybundle};
use crate::reusable::component::bounty::{BountyTitle,TitleImage};
use serde_json::Value;
use serenity::all::*;
use std::collections::HashMap;

#[hertz::hertz_auto]
async fn auto(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    let mut foc = "";
    for i in &bnd.cmd.data.options{
        if let CommandDataOptionValue::Autocomplete { kind:_, value } = &i.value{
            foc = value.as_str();
        }
    }
    let title = BountyTitle::new().await?;
    let mut option = Vec::new();
    for i in title.custom{
        let opt = AutocompleteChoice{name:format!("Trigger on {}",&i.trigger),value:Value::String(i.trigger)};
        option.push(opt);
    }
    option.push(AutocompleteChoice { name: format!("Trigger on {}",&title.bronze_bounty.trigger), value: Value::String(title.bronze_bounty.trigger) });
    option.push(AutocompleteChoice { name: format!("Trigger on {}",&title.silver_bounty.trigger), value: Value::String(title.silver_bounty.trigger) });
    option.push(AutocompleteChoice { name: format!("Trigger on {}",&title.gold_bounty.trigger), value: Value::String(title.gold_bounty.trigger) });
    option.push(AutocompleteChoice { name: format!("Trigger on {}",&title.bronze_trading.trigger), value: Value::String(title.bronze_trading.trigger) });
    option.push(AutocompleteChoice { name: format!("Trigger on {}",&title.silver_trading.trigger), value: Value::String(title.silver_trading.trigger) });
    option.push(AutocompleteChoice { name: format!("Trigger on {}",&title.gold_trading.trigger), value: Value::String(title.gold_trading.trigger) });
    let mut filtered =  option.iter().filter(|x|x.name.contains(foc)).map(|x|x.to_owned()).collect::<Vec<_>>();
    if filtered.len() > 20{
        filtered = filtered[..20].to_vec();
    }
    bnd.cmd.create_response(&bnd.ctx.http, CreateInteractionResponse::Autocomplete(CreateAutocompleteResponse::new().set_choices(filtered))).await?;
    Ok(())
}

#[hertz::hertz_slash_normal(0,true)]
async fn slash(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    let mut item = "";
    for i in &bnd.cmd.data.options{
        if let CommandDataOptionValue::String(x) = &i.value{
            item = x.as_str();
        }
    }
    let title = BountyTitle::new().await?;
    let mut images:HashMap<&str,&TitleImage> = HashMap::new();
    for (i,_) in title.custom.iter().enumerate(){
        images.insert(&title.custom[i].trigger,&title.custom[i].image);
    }
    images.insert(&title.bronze_bounty.trigger,&title.bronze_bounty.image);
    images.insert(&title.silver_bounty.trigger,&title.silver_bounty.image);
    images.insert(&title.gold_bounty.trigger,&title.gold_bounty.image);
    images.insert(&title.bronze_trading.trigger,&title.bronze_trading.image);
    images.insert(&title.silver_trading.trigger,&title.silver_trading.image);
    images.insert(&title.gold_trading.trigger,&title.gold_trading.image);
    let image = images.get(item).ok_or(MyErr::Custom("cant get option value, make sure properly select option".to_owned()))?;
    let att = CreateAttachment::bytes(image.title(bnd.cmd.user.static_avatar_url().unwrap_or(bnd.cmd.user.default_avatar_url()).as_str()).await?, "title.png");
    bnd.cmd.edit_response(&bnd.ctx.http, EditInteractionResponse::new().new_attachment(att)).await?;
    Ok(())
}
