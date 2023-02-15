use std::path::Path;

use serenity::all::*;
use crate::{Init,ErrorLog,Components};

fn get_att(cmd:&CommandInteraction)->Option<Attachment>{
    let resolved = &cmd.data.resolved;
    for i in &cmd.data.options{
        if let CommandDataOptionValue::Attachment(att)= &i.value{
            return Some(resolved.attachments.get(att)?.to_owned());
        }
    }
    None
}
#[derive(Debug)]
enum MyErr{
    Serenity(serenity::Error),
    Tokio(tokio::io::Error),
    Utf8(std::str::Utf8Error),
    Serde(serde_json::Error)
}
impl std::error::Error for MyErr {}
impl std::fmt::Display for MyErr{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            MyErr::Tokio(x)=>x.fmt(f),
            MyErr::Serenity(x)=>x.fmt(f),
            MyErr::Utf8(x)=>x.fmt(f),
            MyErr::Serde(x)=>x.fmt(f)
        }
    }
}
impl From<tokio::io::Error> for MyErr{
    fn from(value: tokio::io::Error) -> Self {
        MyErr::Tokio(value)
    }
}
impl From<serenity::Error> for MyErr {
    fn from(value: serenity::Error) -> Self {
        MyErr::Serenity(value)
    }
}
impl From<std::str::Utf8Error> for MyErr{
    fn from(value: std::str::Utf8Error) -> Self {
        MyErr::Utf8(value)
    }
}
impl From<serde_json::Error> for MyErr {
    fn from(value: serde_json::Error) -> Self {
        MyErr::Serde(value)
    }
}
async fn download_and_save(att:Attachment)->Result<(),MyErr>{
    let byte = att.download().await?;
    let utf8 = std::str::from_utf8(&byte)?;
    let _:super::pull::Gacha =  serde_json::from_str(utf8)?;
    let path = Path::new(".").join("static").join("gacha.json");
    tokio::fs::write(&path, byte).await?;
    Ok(())
}
pub async fn run(ctx:&Context,cmd:&CommandInteraction,init:&Init){
    let mut err = ErrorLog::new(ctx, init, &cmd.user).await;
    let att = match get_att(cmd){
        Some(x)=>x,
        None=>{
            err.change_error("no attachment".to_owned(), "change gacha","idont know, please report".to_owned());
            return err.log_slash(cmd, true).await;
        }
    };
    if &att.filename != "gacha.json"{
        err.change_error("invalid name".to_owned(), "ch_gacha", "send correct file please".to_owned());
        return err.log_slash(cmd, true).await;
    }
    match download_and_save(att).await{
        Ok(_)=>{
            if let Err(why) = cmd.create_response(&ctx.http, Components::interaction_response("banner changed", true)).await{
                err.discord_error(why.to_string(), "confirmation").await;
            }
        }
        Err(why)=>{
            err.change_error(why.to_string(), "download and save then check", "most likely you wrote wrong json config, check the error message, which line is the problem then recheck your json file on the exact line".to_string());
            err.log_slash(cmd, true).await;
        }
    }
}
