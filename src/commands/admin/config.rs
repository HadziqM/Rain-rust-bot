use crate::{MyErr,SlashBundle};
use serenity::all::*;

pub async fn slash(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    let mut name = "";
    for data in &bnd.cmd.data.options{
        if let CommandDataOptionValue::SubCommand(_) = &data.value{
            name = &data.name;
        }
    }
    match name{
        "gacha"=>crate::commands::gacha::ch_gacha::slash(bnd).await?,
        "market"=>crate::commands::market::ch_market::slash(bnd).await?,
        _=>{return Err(MyErr::Custom("you dont have any configuration needed to change".to_owned()))}
    };
    Ok(())
}
