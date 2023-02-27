use crate::{MyErr,SlashBundle,Mybundle,Mytrait,Reg};
use serenity::all::*;
use super::market;

#[hertz::hertz_slash_reg(60,false)]
async fn slash(bnd:&SlashBundle<'_>,reg:&Reg<'_>)->Result<(),MyErr>{
    let mut name = "";
    let mut focus = "";
    let mut auto = false;
    for data in &bnd.cmd.data.options{
        if let CommandDataOptionValue::SubCommand(sub) = &data.value{
            name = &data.name;
            for i in sub{
                if let CommandDataOptionValue::Autocomplete { kind:_, value } = &i.value{
                    auto = true;
                    focus = value.as_str();
                }
            }
        }
    }
    match name{
        "market"=>{
            if auto{
                market::auto(bnd, focus).await?;
            }else{
            market::slash(bnd, reg).await?
            }
        }
        "bar"=>todo!(),
        "restourant"=>todo!(),
        "jewelry"=>todo!(),
        "casino"=>todo!(),
        _=>{return Err(MyErr::Custom("you dont have market enabled".to_owned()))}
    };
    Ok(())
}
