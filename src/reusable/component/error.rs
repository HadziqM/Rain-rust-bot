use std::num::NonZeroU64;
use std::path::Path;

use serenity::all::{CommandInteraction, ComponentInteraction, ModalInteraction};
use serenity::builder::{CreateInteractionResponse, CreateEmbed, CreateInteractionResponseMessage, EditInteractionResponse, CreateMessage, CreateEmbedAuthor, CreateEmbedFooter, CreateAttachment};
use serenity::model::prelude::{ChannelId, UserId};
use serenity::model::user::User;
use serenity::prelude::Context;


use super::super::config::Init;
use super::super::utils::color;

#[derive(Clone)]
pub struct ErrorLog<'a> {
    pub(crate) err: String,
    pub(crate) on:&'a str,
    pub(crate) advice:String,
    pub(crate) ctx:&'a Context,
    pub(crate) init:&'a Init,
    pub(crate) usr:&'a User,
    pub(crate) user:User,
    pub(crate) att:CreateAttachment
}

impl<'a> ErrorLog<'a>{
    pub async fn new(ctx:&'a Context, init:&'a Init,usr:&'a User)->ErrorLog<'a>{
        let user = UserId(NonZeroU64::new(init.discord.author_id).unwrap()).to_user(&ctx.http).await.unwrap_or_default();
        let path = Path::new(".").join("icon").join("panics.png").as_path().to_owned();
        ErrorLog { 
            err: String::new(), 
            on: "", advice:String::new(), 
            ctx, 
            init,
            usr,
            user,
            att:CreateAttachment::path(path).await.unwrap()
        }
    }
    pub fn change_error(&mut self,error:String,on:&'a str,advice:String){
        self.err = error;
        self.on = on;
        self.advice = advice;
    }
    pub fn change_why(&mut self,error:String){
        self.err = error;
    }
    pub fn make_embed(&self)->CreateEmbed{
        CreateEmbed::new()
        .title("🛑 Error Occured 🛑")
        .description("some cant be handled error occured")
        .fields(vec![
            ("🚧 occured on",self.on,false),
            ("📜 error message",&format!("```\n{}\n```",&self.err),false),
            ("⛑  author advice",&self.advice,false)
        ])
        .author(CreateEmbedAuthor::new(&self.usr.name).icon_url(self.usr.face()))
        .footer(CreateEmbedFooter::new(format!("you can consult this to {}",self.user.tag()))
            .icon_url(self.user.face()))
        .color(color("ff", "00", "00"))
        .thumbnail("attachment://panics.png")
    }
    pub async fn log_error_channel(&self){
        let ch_id = ChannelId(NonZeroU64::new(self.init.log_channel.err_channel).unwrap());
        if let Err(why) = ch_id.send_message(&self.ctx.http,CreateMessage::new()
            .embed(self.make_embed()).add_file(self.att.to_owned()).content(format!("for {}",self.usr.to_string()))).await{
            println!("cant send error message to discord channel :{}",why)
        }
    }
    pub fn interaction_response(&self,ephemeral:bool)->CreateInteractionResponse{
        CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().embed(self.make_embed())
            .add_file(self.att.to_owned()).ephemeral(ephemeral))
    }
    pub fn defer_response(&self)->EditInteractionResponse{
        EditInteractionResponse::new().embed(self.make_embed()).new_attachment(self.att.to_owned())
    }
    pub async fn log_slash(&self,cmd:&CommandInteraction,ephemeral:bool){
        if let Err(why) = cmd.create_response(&self.ctx.http,self.interaction_response(ephemeral)).await{
            self.log_error_channel().await;
            println!("{why}");
        }
    }
    pub async fn log_slash_defer(&self,cmd:&CommandInteraction,_ephemeral:bool){
        if let Err(why) = cmd.edit_response(&self.ctx.http,EditInteractionResponse::new().embed(self.make_embed()).new_attachment(self.att.to_owned())).await{
            self.log_error_channel().await;
            println!("{why}");
        }
    }
    pub async fn log_button(&self,cmd:&ComponentInteraction,ephemeral:bool){
        if let Err(why) = cmd.create_response(&self.ctx.http,
            self.interaction_response(ephemeral)).await{
            self.log_error_channel().await;
            println!("{why}");
        }
    }
    pub async fn log_modal(&self,cmd:&ModalInteraction,ephemeral:bool){
        if let Err(why) = cmd.create_response(&self.ctx.http,
            self.interaction_response(ephemeral)).await{
            self.log_error_channel().await;
            println!("{why}");
        }
    }
    pub async fn discord_error(&mut self,error:String,on:&'a str){
        self.change_error(error, on, "discord connection problem, you can consult this problem".to_string());
        self.log_error_channel().await;
    }
    pub async fn pgcon_error(&mut self,error:String,on:&'a str,cmd:&CommandInteraction){
        self.change_error(error, on, "connection to database timedout, wait for server to be stable".to_string());
        self.log_slash(cmd, false).await;
    }
    pub async fn pgcon_error_defer(&mut self,error:String,on:&'a str,cmd:&CommandInteraction){
        self.change_error(error, on, "connection to database timedout, wait for server to be stable".to_string());
        self.log_slash_defer(cmd, false).await;
    }
    pub async fn pgcon_error_ch(&mut self,error:String,on:&'a str){
        self.change_error(error, on, "connection to database timedout, wait for server to be stable".to_string());
        self.log_error_channel().await;
    }
    pub async fn pgcon_error_button(&mut self,error:String,on:&'a str,cmd:&ComponentInteraction){
        self.change_error(error, on, "connection to database timedout, wait for server to be stable".to_string());
        self.log_button(cmd, true).await;
    }
}
