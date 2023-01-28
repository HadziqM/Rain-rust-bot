use serenity::{model::prelude::interaction::{application_command::ApplicationCommandInteraction, message_component::MessageComponentInteraction, modal::ModalSubmitInteraction}, prelude::Context};
use crate::commands;
use crate::reusable::config::Init;

pub async fn slash_command(cmd_id:&str,cmd:&ApplicationCommandInteraction,ctx:&Context,hnd:&Init){
    match cmd_id{
        "interface"=>commands::admin::interface::run(ctx,cmd,hnd).await,
        "create"=>commands::register::create::run_slash(ctx,cmd,hnd).await,
        "check"=>commands::register::check::run(ctx,cmd,hnd).await,
        "change_password"=>commands::register::change_pasword::run(ctx,cmd,hnd).await,
        "card"=>commands::binded::card::run(ctx,cmd,hnd).await,
        "switch"=>commands::register::bind::run(ctx,cmd,hnd).await,
        "Card"=>commands::binded::card::run_user(ctx,cmd,hnd).await,
        "dm_save"=>commands::binded::save::run(ctx,cmd,hnd).await,
        _=> {return;}
    }
}
pub async fn button_command(cmd_id:&str,cmd:&MessageComponentInteraction,ctx:&Context,hnd:&Init){
    match cmd_id{
        "register_i"=>commands::register::create::run_button(ctx, cmd,hnd).await,
        // "bind_i"=>commands::register::button::bind(ctx, cmd).await,
        // "transfer_i"=>commands::register::button::transfer(ctx, cmd).await,
        // "dm_save_i"=>commands::register::button::dm_save(ctx, cmd).await,
        _=>{return;}
    }
}
pub async fn modal_command(cmd_id:&str,cmd:&ModalSubmitInteraction,ctx:&Context,hnd:&Init){
    match cmd_id{
        "register_m"=>commands::register::create::modal_register(ctx,cmd,hnd).await,
        _=>{return;}
    }
}
