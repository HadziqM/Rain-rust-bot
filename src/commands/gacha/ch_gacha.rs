use std::path::Path;
use serenity::all::*;
use crate::{Init,ErrorLog,Components};

pub async fn run(ctx:&Context,cmd:&CommandInteraction,init:&Init){
    let mut err = ErrorLog::new(ctx, init, &cmd.user).await;
    let path = Path::new(".").join("static").join("gacha.json");
    let att = match Components::get_att(cmd){
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
    match Components::download_check_and_save(att,&path,&super::pull::Gacha::default()).await{
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
