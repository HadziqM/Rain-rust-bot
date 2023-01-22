use serenity::{model::prelude::interaction::{application_command::ApplicationCommandInteraction, message_component::MessageComponentInteraction, modal::ModalSubmitInteraction}, prelude::Context};
use crate::commands;


pub async fn slash_command(cmd_id:&str,cmd:&ApplicationCommandInteraction,ctx:&Context){
    match cmd_id{
        "ping" => commands::ping::run(&cmd.data.options,ctx,cmd).await,
        "id" =>commands::id::run(&cmd.data.options,ctx,cmd).await,
        "interface"=>commands::register::interface::run(&cmd.data.options, ctx, cmd).await,
        _=> println!("slash command {} isnt handled yet",cmd_id),
    }
}
pub async fn button_command(cmd_id:&str,cmd:&MessageComponentInteraction,ctx:&Context){
    match cmd_id{
        "register_i"=>commands::register::button::register(ctx, cmd).await,
        "bind_i"=>commands::register::button::bind(ctx, cmd).await,
        "transfer_i"=>commands::register::button::transfer(ctx, cmd).await,
        "dm_save_i"=>commands::register::button::dm_save(ctx, cmd).await,
        _=>println!("button {} isnt handled yet",cmd_id)
    }
}
pub async fn modal_command(cmd_id:&str,cmd:&ModalSubmitInteraction,ctx:&Context){
    match cmd_id{
        "register_m"=>commands::register::button::modal_register(ctx, cmd, &cmd.data).await,
        _=>println!("modal {} isnt handled yet",cmd_id)
    }
}
