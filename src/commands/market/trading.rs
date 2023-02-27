use crate::{MyErr,SlashBundle,Mybundle,Mytrait,Reg};
use serenity::all::*;
use super::market;

#[hertz::hertz_slash_reg(60,false)]
async fn slash(bnd:&SlashBundle<'_>,reg:&Reg<'_>)->Result<(),MyErr>{
    let mut name = "";
    for data in &bnd.cmd.data.options{
        if let CommandDataOptionValue::SubCommand(_) = &data.value{
            name = &data.name;
        }
    }
    match name{
        "market"=>market::slash(bnd, reg).await?,
        "bar"=>todo!(),
        "restourant"=>todo!(),
        "jewelry"=>todo!(),
        "casino"=>todo!(),
        _=>{return Err(MyErr::Custom("you dont have market enabled".to_owned()))}
    };
    Ok(())
}

#[hertz::hertz_auto]
async fn auto(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    let mut name = "";
    let mut focus = "";
    for data in &bnd.cmd.data.options{
        if let CommandDataOptionValue::SubCommand(sub) = &data.value{
            name = &data.name;
            for i in sub{
                if let CommandDataOptionValue::Autocomplete { kind:_, value } = &i.value{
                    focus = value.as_str();
                }
            }
        }
    }
    match name{
        "market"=>market::auto(bnd, focus).await?,
        "bar"=>todo!(),
        "restourant"=>todo!(),
        "jewelry"=>todo!(),
        "casino"=>todo!(),
        _=>{return Err(MyErr::Custom("you dont have market enabled".to_owned()))}
    };
    Ok(())
}
