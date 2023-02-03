use serenity::all::{CommandInteraction, ComponentInteraction, ModalInteraction};
use serenity::prelude::Context;

use crate::commands;
use crate::reusable::config::Init;

pub async fn slash_command(cmd_id:&str,cmd:&CommandInteraction,ctx:&Context,hnd:&Init){
    match cmd_id{
        "interface"=>commands::admin::interface::run(ctx,cmd,hnd).await,
        "create"=>commands::register::create::run_slash(ctx,cmd,hnd).await,
        "check"=>commands::register::check::run(ctx,cmd,hnd).await,
        "change_password"=>commands::register::change_pasword::run(ctx,cmd,hnd).await,
        "card"=>commands::binded::card::run(ctx,cmd,hnd).await,
        "switch"=>commands::register::bind::run(ctx,cmd,hnd).await,
        "👤 Card"=>commands::binded::card::run_user(ctx,cmd,hnd).await,
        "dm_save"=>commands::binded::save::run(ctx,cmd,hnd).await,
        "transfer"=>commands::binded::transfer::run(ctx,cmd,hnd).await,
        "reset_save_cd"=>commands::admin::save_cd::run(ctx,cmd,hnd).await,
        _=> {return;}
    }
}
pub async fn button_command(cmd_id:&str,cmd:&ComponentInteraction,ctx:&Context,hnd:&Init){
    //if its a dynamic id's
    let dynamic = cmd_id.split("_").collect::<Vec<_>>();
    if dynamic.contains(&"save"){
        commands::binded::transfer::run_button(dynamic, ctx, cmd,hnd).await;
    }else{
        match cmd_id{
            "register"=>commands::register::create::run_button(ctx, cmd,hnd).await,
            "dms"=>commands::binded::save::run_button(ctx, cmd,hnd).await,
            // "transfer_i"=>commands::register::button::transfer(ctx, cmd).await,
            // "dm_save_i"=>commands::register::button::dm_save(ctx, cmd).await,
            _=>{return;}
        }
    }
}
pub async fn modal_command(cmd_id:&str,cmd:&ModalInteraction,ctx:&Context,hnd:&Init){
    match cmd_id{
        "register_m"=>commands::register::create::modal_register(ctx,cmd,hnd).await,
        _=>{return;}
    }
}
