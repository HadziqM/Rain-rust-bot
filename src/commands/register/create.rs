use serenity::builder::{CreateActionRow, CreateInteractionResponse, CreateApplicationCommand};
use serenity::model::prelude::component::{InputTextStyle, ActionRowComponent};
use serenity::model::prelude::interaction::InteractionResponseType::Modal;
use serenity::model::prelude::interaction::application_command::{ApplicationCommandInteraction, CommandDataOption};
use serenity::model::prelude::interaction::message_component::MessageComponentInteraction;
use serenity::model::application::interaction::modal::{ModalSubmitInteraction,ModalSubmitInteractionData};
use serenity::prelude::Context;

use crate::reusable::config::Init;
use crate::reusable::component::error::*;
use crate::reusable::postgress::PgConn;

fn modal_register_row(name:&str,pass:bool)->CreateActionRow{
    let placeholder = match pass {
        false => "your MHFZ username on launcher".to_owned(),
        true => "your MHFZ user password (igonore discord warning)".to_owned(),
    };
    let mut row = CreateActionRow::default();
    row.create_input_text(|i|{
        i.label(name)
         .custom_id(name)
         .required(true)
         .style(InputTextStyle::Short)
         .placeholder(&placeholder)
    });
    row
}

fn modal_response<'a,'b>(lt:&'a mut CreateInteractionResponse<'b>)->&'a mut CreateInteractionResponse<'b>{
    lt.kind(Modal)
       .interaction_response_data(|m|{
            m.components(|c|c.add_action_row(modal_register_row("username",false))
               .add_action_row(modal_register_row("password", true)))
                .custom_id("register_m")
                .title("register command")
    })
}


pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("create").description("to create account on rain mhfz server")
}
pub async fn run_button(ctx:&Context,cmd:&MessageComponentInteraction,init:&Init){
    if let Err(why) = cmd.create_interaction_response(&ctx.http, |r|{
        modal_response(r)
    }).await{
        error(ctx, &why.to_string(), "register interface button", "most likely discord connection problem, so modal didnt shown, try consult this one or wait till connection is more stable",init,&cmd.user).await;
    }
}
pub async fn run_slash(_options: &[CommandDataOption],ctx:&Context, cmd:&ApplicationCommandInteraction,init:&Init){
    if let Err(why) = cmd.create_interaction_response(&ctx.http, |r|{
        modal_response(r)
    }).await{
        error(ctx, &why.to_string(), "register command", "most likely discord connection problem, so modal didnt shown, try consult this one or wait till connection is more stable",init,&cmd.user).await;
    }
}

pub async fn modal_register(ctx:&Context,cmd:&ModalSubmitInteraction,data:&ModalSubmitInteractionData,init:&Init){
    let mut name = String::new();
    let mut password = String::new();
    for comp in &data.components{
        let arow = comp.components.first().unwrap();
        if let ActionRowComponent::InputText(input) = arow{
            match input.custom_id.as_str() {
                "username" => name = input.value.to_owned(),
                 _=> password = input.value.to_owned(),
            }
        }
    }
    match PgConn::create(init,&cmd.user.id.to_string()).await{
        Ok(pg)=>{
            match pg.create_account(&name, &password).await{
                Ok(_id)=>{
                    error_modal(ctx,"idk", "create modal submition", "failed to create account, most likely account already used or you use special character on username", cmd, init).await
                }
                Err(why)=>{
                    let err = why.to_string();
                    error_modal(ctx,&err, "create modal submition", "failed to create account, most likely account already used or you use special character on username", cmd, init).await
                }
            }
        }
        Err(err)=>{
            error_modal(ctx,&err.to_string(), "create modal submition", "error connecting to database wait for few minute for server to back", cmd, init).await
        }
    }
}
