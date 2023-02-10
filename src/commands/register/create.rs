use std::num::NonZeroU64;

use serenity::builder::{CreateActionRow, CreateInteractionResponse};
use serenity::model::prelude::{RoleId, Member, ChannelId};
use serenity::prelude::Context;
use crate::reusable::utils::color;
use crate::{PgConn,Init,ErrorLog,Components};
use serenity::all::*;

struct RegisterAcknowledged<'a,'b> {
    username: &'a str,
    user:&'a mut Member,
    uid:i32,
    ctx:&'a Context,
    err:&'a mut ErrorLog<'b>,
}

impl<'a,'b> RegisterAcknowledged<'a,'b>{
    fn new(username:&'a str,user:&'a mut Member,uid:i32,ctx:&'a Context,err:&'a mut ErrorLog<'b>)->RegisterAcknowledged<'a,'b>{
        RegisterAcknowledged { username, user, uid,ctx,err }
    }
    async fn add_roles(&mut self){
        let rid = RoleId(NonZeroU64::new(self.err.init.server_role.register_role).unwrap());
        if let Err(why)=self.user.add_role(&self.ctx.http,rid).await{
            self.err.change_error(why.to_string(), "add register role", "ask admin to give you role manually".to_string());
            self.err.log_error_channel().await;
        }
    }
    async fn log_to_user(&mut self,cmd:&ModalInteraction,reg:bool){
        let word = || {if reg{return "Created";}"binded"};
        let serv =cmd.guild_id.unwrap_or_default();
        let server = serv.to_partial_guild(&self.ctx.http).await.unwrap();
        let ch = ChannelId(NonZeroU64::new(self.err.init.log_channel.account_channel).unwrap());
        if let Err(why) = ch.send_message(&self.ctx.http,CreateMessage::new().embed(CreateEmbed::new().title(format!("Account Succesfully {} on Server",word()))
                .description(&format!("{} {} an account on server, by creating account here you already aggree to follow our rules to as stated on rules channel, as a member of {} comunity we welcome you to enjoy the game together",self.user.to_string(),word(),server.name)).fields(vec![
                    ("👤 Username",&format!("`{}`",self.username),false),
                    ("🆔 User Id",&format!("`{}`",self.uid),false)
                ]).author(CreateEmbedAuthor::new(&self.user.user.name).icon_url(self.user.face()))
                .colour(color("00", "ff", "00"))
                .image("https://media.discordapp.net/attachments/1068440173479739393/1068458599627620392/cachedImage.png?width=807&height=455"))).await{
            self.err.change_error(why.to_string(), "log user create", "sorry connection problem we cant send your greeting message".to_string());
            self.err.log_error_channel().await;
        }
    }
    async fn send_response(&mut self,cmd:&ModalInteraction){
        if let Err(why) = cmd.create_response(&self.ctx.http, Components::interaction_response("account succesfully created", true)).await{
            self.err.change_error(why.to_string(), "responding modal", "account succesfully created so dont worry, its just discord connection problem".to_string());
            self.err.log_error_channel().await;
        }
    }
}
fn modal_register_row(name:&str,pass:bool)->CreateActionRow{
    let placeholder = match pass {
        false => "your MHFZ username on launcher".to_owned(),
        true => "your MHFZ user password (igonore discord warning)".to_owned(),
    };
    CreateActionRow::InputText(
        CreateInputText::new(InputTextStyle::Short, name, name).required(true).placeholder(&placeholder)
    )
}

fn modal_response(reg:bool)->CreateInteractionResponse{
    let name;
    let title;
    if reg{
        name="register_m";
        title="Register Command";
    }else{
        name="bind_m";
        title="Bind Command";
    }
    CreateInteractionResponse::Modal(CreateModal::new(name, title)
        .components(vec![modal_register_row("username", false),modal_register_row("password", true)]))
}
pub async fn run_button(ctx:&Context,cmd:&ComponentInteraction,init:&Init,regist:bool){
    let mut err = ErrorLog::new(&ctx, init, &cmd.user).await;
    let did = cmd.user.id.to_string();
    match PgConn::create(init, did).await {
        Ok(mut pg) =>{
            match pg.get_user_data().await {
                Ok(data) => {
                    if data.cid != 0 || data.rid!=0{
                        err.change_error("you already have account in game".to_string(), "checking user data", "you cant have more than one account sorry".to_string());
                        err.log_button(cmd, true).await;
                        return pg.close().await;
                    }
                    pg.close().await;
                }
                Err(why) => {
                    err.change_error(why.to_string(), "getting user data", "please report this".to_string());
                    err.log_button(cmd, true).await;
                    return pg.close().await;
                }
            }
        }
        Err(why) => {
            err.pgcon_error_button(why.to_string(), "create button", cmd).await;
            return;
        }
    };
    if let Err(why) = cmd.create_response(&ctx.http,modal_response(regist)).await{
        err.change_error(why.to_string(), "register interface button", "failed to response, most likely your registrasion already done, its just discord error".to_string());
        err.log_error_channel().await;
    }
}
pub async fn run_slash(ctx:&Context, cmd:&CommandInteraction,init:&Init,regist:bool){
    let mut err = ErrorLog::new(&ctx, init, &cmd.user).await;
    let did = cmd.user.id.to_string();
    match PgConn::create(init, did).await {
        Ok(mut pg) =>{
            match pg.get_user_data().await {
                Ok(data) => {
                    if data.cid != 0 || data.rid!=0{
                        err.change_error("you already have account in game".to_string(), "checking user data", "you cant have more than one account sorry".to_string());
                        err.log_slash(cmd, false).await;
                        return pg.close().await;
                    }
                    pg.close().await;
                }
                Err(why) => {
                    err.change_error(why.to_string(), "getting user data", "please report this".to_string());
                    err.log_slash(cmd, false).await;
                    return pg.close().await;
                }
            }
        }
        Err(why) => {
            err.pgcon_error(why.to_string(), "create button", cmd).await;
            return;
        }
    };
    if let Err(why) = cmd.create_response(&ctx.http,modal_response(regist)).await{
        let mut err = ErrorLog::new(&ctx, init, &cmd.user).await;
        err.change_error(why.to_string(), "register interface button", "failed to response, most likely your registrasion already done, its just discord error".to_string());
        err.log_error_channel().await;
    }
}
pub async fn modal_register(ctx:&Context,cmd:&ModalInteraction,init:&Init,regist:bool){
    let data = cmd.data.to_owned();
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
    match PgConn::create(init, cmd.user.id.to_string()).await{
        Ok(mut pg)=>{
            match pg.create_account(&name, &password,regist).await{
                Ok(cid)=>{
                    let mut member = cmd.member.to_owned().unwrap();
                    let mut reg = RegisterAcknowledged::new(&name,&mut member, cid.id, ctx, &mut error);
                    reg.send_response(cmd).await;
                    reg.add_roles().await;
                    reg.log_to_user(cmd,regist).await;
                }
                Err(why)=>{
                    error.change_error(why.to_string(), "submit register", "failed to create account, maybe the user is already taken, and dont use special character like `'` on name or password".to_string());
                    error.log_modal(cmd,true).await;
                }
            }
            pg.close().await;
        }
        Err(err)=>{
            error.change_error(err.to_string(), "submit register", "database connection timedout, wait for few minutes or maintenance finished".to_string());
            error.log_modal(cmd,true).await;
        }
    }
}
