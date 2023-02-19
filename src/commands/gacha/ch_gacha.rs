use std::path::Path;
use serenity::all::*;
use crate::{SlashBundle,MyErr,Components};

pub async fn slash(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    let path = Path::new(".").join("static").join("gacha.json");
    let att = Components::get_att(bnd.cmd)?;
    Components::download_check_and_save(att,&path,&super::pull::Gacha::default()).await?;
    Components::edit(bnd, "gacha changed").await?;
    Ok(())
}
