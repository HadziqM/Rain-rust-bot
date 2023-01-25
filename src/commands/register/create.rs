use serenity::builder::{CreateActionRow, CreateInteractionResponse, CreateApplicationCommand};
use serenity::model::prelude::{RoleId, Member, ChannelId};
use serenity::model::prelude::component::{InputTextStyle, ActionRowComponent};
use serenity::model::prelude::interaction::InteractionResponseType::*;
use serenity::model::prelude::interaction::application_command::{ApplicationCommandInteraction, CommandDataOption};
use serenity::model::prelude::interaction::message_component::MessageComponentInteraction;
use serenity::model::application::interaction::modal::{ModalSubmitInteraction,ModalSubmitInteractionData};
use serenity::prelude::Context;
use crate::reusable::utils::color;
use crate::{PgConn,Init,ErrorLog};

struct RegisterAcknowledged<'a,'b> {
    username: &'a str,
    user:&'a mut Member,
    uid:i64,
    ctx:&'a Context,
    err:&'a mut ErrorLog<'b>,
}

impl<'a,'b> RegisterAcknowledged<'a,'b>{
    fn new(username:&'a str,user:&'a mut Member,uid:i64,ctx:&'a Context,err:&'a mut ErrorLog<'b>)->RegisterAcknowledged<'a,'b>{
        RegisterAcknowledged { username, user, uid,ctx,err }
    }
    async fn add_roles(&mut self){
        let rid = RoleId(self.err.init.server_role.register_role);
        if let Err(why)=self.user.add_role(&self.ctx.http,rid).await{
            self.err.change_error(why.to_string(), "add register role", "ask admin to give you role manually");
            self.err.log_error_channel().await;
        }
    }
    async fn log_to_user(&mut self){
        let ch = ChannelId(self.err.init.log_channel.account_channel);
        if let Err(why) = ch.send_message(&self.ctx.http, |m|{
            m.embed(|e|{
                e.title("Account Succesfully Created on Server")
                .description(&format!("{} created an account on server, by creating account here you already aggree to follow our rules to as stated on rules channel, as a member of rain server comunity we welcome you to enjoy the game together",self.user.to_string())).fields(vec![
                    ("👤 Username",&format!("`{}`",self.username),false),
                    ("🆔 User Id",&format!("`{}`",self.uid),false)
                ]).author(|a|a.name(self.user.display_name()).icon_url(self.user.face()))
                .colour(color("00", "ff", "00"))
                .image("attachment://server.jpg")
            }).add_file("./misc/server.jpg")
        }).await{
            self.err.change_error(why.to_string(), "log user create", "sorry connection problem we cant send your greeting message");
            self.err.log_error_channel().await;
        }
    }
    async fn send_response(&mut self,cmd:&ModalSubmitInteraction){
        if let Err(why) = cmd.create_interaction_response(&self.ctx.http, |m|{
            m.kind(ChannelMessageWithSource).interaction_response_data(|i|{
                i.ephemeral(true).content("account succesfully created")
            })
        }).await{
            self.err.change_error(why.to_string(), "responding modal", "account succesfully created so dont worry, its just discord connection problem")
        }
    }
}
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

pub async fn modal_register(ctx:&Context,cmd:&mut ModalSubmitInteraction,data:&ModalSubmitInteractionData,init:&Init){
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
    match PgConn::create(init, &cmd.user.id.to_string()).await{
        Ok(mut pg)=>{
            match pg.create_account(&name, &password).await{
                Ok(id)=>{
                    match id {
                        Some(cid) => {
                            let mut member = cmd.member.to_owned().unwrap();
                            let mut reg = RegisterAcknowledged::new(&name,&mut member, cid, ctx, &mut error);
                            reg.send_response(cmd).await;
                            reg.add_roles().await;
                            reg.log_to_user().await;
                        }
                        None => {
                            error.change_error("no error message".to_string(), "submit register", "you already have account on the server run `/check` to check your username,`/change_pass` to change your password");
                            error.log_modal(cmd,true).await;
                        }
                    }
                }
                Err(why)=>{
                    error.change_error(why.to_string(), "submit register", "failed to create account, maybe the user is already taken, and dont use special character like `'` on name or password");
                    error.log_modal(cmd,true).await;
                }
            }
            pg.close().await;
        }
        Err(err)=>{
            error.change_error(err.to_string(), "submit register", "database connection timedout, wait for few minutes or maintenance finished");
            error.log_modal(cmd,true).await;
        }
    }
}
