use serenity::{model::prelude::interaction::{application_command::ApplicationCommandInteraction, message_component::MessageComponentInteraction, modal::ModalSubmitInteraction}, prelude::Context};
use crate::commands;
use crate::reusable::config::Init;

pub async fn slash_command(cmd_id:&str,cmd:&ApplicationCommandInteraction,ctx:&Context,hnd:&Init){
    match cmd_id{
        "ping" => commands::ping::run(&cmd.data.options,ctx,cmd).await,
        "id" =>commands::id::run(&cmd.data.options,ctx,cmd).await,
        "error"=>commands::error::run(&cmd.data.options, ctx, cmd,hnd).await,
        "interface"=>commands::register::interface::run(&cmd.data.options,ctx,cmd,hnd).await,
        _=> println!("slash command {} isnt handled yet",cmd_id),
    }
}
pub async fn button_command(cmd_id:&str,cmd:&MessageComponentInteraction,ctx:&Context,hnd:&Init){
    match cmd_id{
        "register_i"=>commands::register::button::register(ctx, cmd,hnd).await,
        // "bind_i"=>commands::register::button::bind(ctx, cmd).await,
        // "transfer_i"=>commands::register::button::transfer(ctx, cmd).await,
        // "dm_save_i"=>commands::register::button::dm_save(ctx, cmd).await,
        _=>println!("button {} isnt handled yet",cmd_id)
    }
}
pub async fn modal_command(cmd_id:&str,cmd:&ModalSubmitInteraction,ctx:&Context,hnd:&Init){
    match cmd_id{
        "register_m"=>commands::register::button::modal_register(ctx, cmd, &cmd.data,hnd).await,
        _=>println!("modal {} isnt handled yet",cmd_id)
    }
}
