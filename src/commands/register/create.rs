use serenity::builder::{CreateActionRow, CreateInteractionResponse, CreateApplicationCommand};
use serenity::model::prelude::component::{InputTextStyle, ActionRowComponent};
use serenity::model::prelude::interaction::InteractionResponseType::Modal;
use serenity::model::prelude::interaction::application_command::{ApplicationCommandInteraction, CommandDataOption};
use serenity::model::prelude::interaction::message_component::MessageComponentInteraction;
use serenity::model::application::interaction::modal::{ModalSubmitInteraction,ModalSubmitInteractionData};
use serenity::prelude::Context;
use crate::{PgConn,Init,ErrorLog};

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
    let mut err = ErrorLog::new(&ctx, init, &cmd.user).await;
    if let Err(why) = cmd.create_interaction_response(&ctx.http, |r|{
        modal_response(r)
    }).await{
        err.change_error(why.to_string(), "register interface button", "failed to response, most likely your registrasion already done, its just discord error");
        err.log_error_channel().await;
    }
}
pub async fn run_slash(_options: &[CommandDataOption],ctx:&Context, cmd:&ApplicationCommandInteraction,init:&Init){
    if let Err(why) = cmd.create_interaction_response(&ctx.http, |r|{
        modal_response(r)
    }).await{
        let mut err = ErrorLog::new(&ctx, init, &cmd.user).await;
        err.change_error(why.to_string(), "register interface button", "failed to response, most likely your registrasion already done, its just discord error");
        err.log_error_channel().await;
    }
}

pub async fn modal_register(ctx:&Context,cmd:&ModalSubmitInteraction,data:&ModalSubmitInteractionData,init:&Init){
    let mut error = ErrorLog::new(&ctx, init, &cmd.user).await;
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
                    println!("idk");
                }
                Err(why)=>{
                    error.change_error(why.to_string(), "submit register", "failed to create account, maybe the user is already taken, and dont use special character like `'` on name or password");
                    error.log_modal(&cmd).await;
                }
            }
        }
        Err(err)=>{
            error.change_error(err.to_string(), "submit register", "database connection timedout, wait for few minutes or maintenance finished");
            error.log_modal(&cmd).await;
        }
    }
}
