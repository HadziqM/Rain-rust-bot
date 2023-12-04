use std::num::NonZeroU64;
use std::path::Path;

use serenity::all::*;
use super::super::config::Init;
use super::super::utils::Color;

#[derive(Clone)]
pub struct ErrorLog<'a> {
    pub(crate) err: String,
    pub(crate) on:String,
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
            on: String::new(), advice:String::new(), 
            ctx, 
            init,
            usr,
            user,
            att:CreateAttachment::path(path).await.unwrap()
        }
    }
    pub fn change_error(&mut self,error:String,on:String,advice:String){
        self.err = error;
        self.on = on;
        self.advice = advice;
    }
    pub fn change_why(&mut self,error:String){
        self.err = error;
    }
    pub fn make_embed(&self,severity:bool)->CreateEmbed{
        let color = ||{if severity{return Color::Red.throw();}Color::Yellow.throw()};
        CreateEmbed::new()
        .title("🛑 Error Occured 🛑")
        .description("some cant be handled error occured")
        .fields(vec![
            ("🚧 occured on",format!("**{}**",self.on.to_uppercase()),false),
            ("📜 error message",format!("> {}",&self.err),false),
            ("⛑  author advice",format!("```\n{}\n```",&self.advice),false)
        ])
        .author(CreateEmbedAuthor::new(&self.usr.name).icon_url(self.usr.face()))
        .footer(CreateEmbedFooter::new(format!("you can consult this to {}",self.user.tag()))
            .icon_url(self.user.face()))
        .color(color())
        .thumbnail("attachment://panics.png")
    }
    pub async fn log_error_channel(&self,severity:bool){
        let ch_id = ChannelId(NonZeroU64::new(self.init.log_channel.err_channel).unwrap());
        if let Err(why) = ch_id.send_message(&self.ctx.http,CreateMessage::new()
            .embed(self.make_embed(severity)).add_file(self.att.to_owned()).content(format!("for {}",self.usr.to_string()))).await{
            println!("cant send error message to discord channel :{}",why)
        }
    }
    pub fn interaction_response(&self,ephemeral:bool,severity:bool)->CreateInteractionResponse{
        CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().embed(self.make_embed(severity))
            .add_file(self.att.to_owned()).ephemeral(ephemeral))
    }
    pub fn defer_response(&self,severity:bool)->EditInteractionResponse{
        EditInteractionResponse::new().embed(self.make_embed(severity)).new_attachment(self.att.to_owned())
    }
}
