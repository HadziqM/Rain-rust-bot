use serenity::all::CommandDataOptionValue;
use serenity::builder::CreateInteractionResponseMessage;

use crate::{Mytrait,Mybundle,MyErr,SlashBundle,Components};
use crate::reusable::component::bounty::{BBQ,Category,Bounty};
#[hertz::hertz_slash_normal(60,false)]
async fn slash(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    let mut name = "";
    for sub in &bnd.cmd.data.options{
        if let CommandDataOptionValue::SubCommand(_) = &sub.value{
            name = sub.name.as_str();
        }
    }
    match name{
        "submit"=>super::submit::submit(bnd).await?,
        "pedia"=>pedia(bnd).await?,
        _ => {return Err(MyErr::Custom("cant get the correct sub name".to_owned()))}
    };
    Ok(())
}

async fn pedia(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    let mut category = "";
    let mut bbq = "";
    for i in Components::sub_options(bnd)?{
        if let CommandDataOptionValue::String(x) = &i.value{
            match i.name.as_str(){
                "category"=>{category = &x},
                "bbq"=>{bbq = &x},
                _ => {continue;}
            }
        }
    }
    let cat = Category::new(category.parse::<u8>().unwrap())?;
    let bb = BBQ::new(bbq.parse::<u8>().unwrap())?;
    let bounty = Box::new(Bounty::new(&cat).await?);
    let embed = bounty.desc(bnd, &bb,&cat)?;
    Components::response_adv(bnd, serenity::builder::CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().embed(embed))).await?;
    Ok(())
}
