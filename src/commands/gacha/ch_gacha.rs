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
    Tokio(tokio::io::Error)
}
impl std::error::Error for MyErr {}
impl std::fmt::Display for MyErr{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            MyErr::Tokio(x)=>x.fmt(f),
            MyErr::Serenity(x)=>x.fmt(f)
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
async fn download_and_save(att:Attachment)->Result<(),MyErr>{
    let byte = att.download().await?;
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
            err.change_error(why.to_string(), "download and save", "please report the error".to_string());
            err.log_slash(cmd, true).await;
        }
    }
}
