use std::path::Path;
use serenity::all::*;
use crate::{SlashBundle,MyErr,Components};
use super::market::Market;

pub async fn slash(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    let path = Path::new(".").join("static").join("market.json");
    let att = Components::get_att(bnd.cmd)?;
    Components::download_check_and_save(att,&path,&Market::default()).await?;
    Components::edit(bnd, "market changed").await?;
    let mark = Market::new().await?;
    mark.update(bnd.ctx, bnd.init, bnd.pedia).await?;
    Ok(())
}
