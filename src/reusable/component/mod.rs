pub mod error;
pub mod widget;
pub mod guide;
pub mod card;
pub mod registered;
pub mod discord;
pub mod json;
pub mod new_error;
pub mod bounty;
pub mod gacha;
pub mod market;

pub struct Components;

use serenity::async_trait;
use serenity::all::*;
use error::ErrorLog;

#[derive(Debug)]
pub enum MyErr{
    Serenity(serenity::Error),
    Tokio(tokio::io::Error),
    Utf8(std::str::Utf8Error),
    Serde(serde_json::Error),
    ByteWise(super::bitwise::BitwiseError),
    Image(super::image_edit::CustomImageError),
    Custom(String)
}
impl std::error::Error for MyErr {}
impl std::fmt::Display for MyErr{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            MyErr::Tokio(x)=>x.fmt(f),
            MyErr::Serenity(x)=>x.fmt(f),
            MyErr::Utf8(x)=>x.fmt(f),
            MyErr::Serde(x)=>x.fmt(f),
            MyErr::Custom(x)=>x.fmt(f),
            MyErr::ByteWise(x)=>x.fmt(f),
            MyErr::Image(x)=>x.fmt(f)
        }
    }
}
impl From<super::bitwise::BitwiseError> for MyErr {
    fn from(value: super::bitwise::BitwiseError) -> Self {
        MyErr::ByteWise(value)
    }
}
impl From<tokio::io::Error> for MyErr{
    fn from(value: tokio::io::Error) -> Self {
        MyErr::Tokio(value)
    }
}
impl From<serenity::Error> for MyErr {
    fn from(value: serenity::Error) -> Self {
        MyErr::Serenity(value)
    }
}
impl From<std::str::Utf8Error> for MyErr{
    fn from(value: std::str::Utf8Error) -> Self {
        MyErr::Utf8(value)
    }
}
impl From<serde_json::Error> for MyErr {
    fn from(value: serde_json::Error) -> Self {
        MyErr::Serde(value)
    }
}
impl From<super::image_edit::CustomImageError> for MyErr {
    fn from(value: super::image_edit::CustomImageError) -> Self {
        MyErr::Image(value)
    }
}

#[async_trait]
pub trait Mytrait{
    async fn err_response(&self,err:&ErrorLog<'_>,ephemeral:bool);
    async fn err_defer(&self,err:&ErrorLog<'_>);
    async fn defer_res(&self,err:&mut ErrorLog<'_>,on:&str,ephemeral:bool);
    async fn response(&self,ctx:&Context,rply:CreateInteractionResponse)->Result<(),MyErr>;
    async fn get_msg(&self,ctx:&Context)->Result<Message,MyErr>;
    async fn edit(&self,ctx:&Context,rlpy:EditInteractionResponse)->Result<(),MyErr>;
    fn user(&self)->User;
}

#[async_trait]
impl Mytrait for CommandInteraction{
    async fn err_response(&self,err:&ErrorLog<'_>,ephemeral:bool){
        if let Err(why) = self.create_response(&err.ctx.http, err.interaction_response(ephemeral,false)).await{
            MyErr::from(why).log_channel(err).await
        }
    }
    async fn err_defer(&self,err:&ErrorLog<'_>){
        if let Err(why) = self.edit_response(&err.ctx.http, err.defer_response(false)).await{
            MyErr::from(why).log_channel(err).await
        }
    }
    async fn defer_res(&self,err:&mut ErrorLog<'_>,on:&str,ephemeral:bool){
        let res;
        if ephemeral{
            res = self.defer_ephemeral(&err.ctx.http).await;
        }else{
            res = self.defer(&err.ctx.http).await;
        }
        if let Err(why) = res{
            let er = MyErr::from(why);
            err.change_error(er.get(),on.to_owned(), er.advice());
            err.log_error_channel(er.severity()).await;
        }
    }
    async fn response(&self,ctx:&Context,rply:CreateInteractionResponse)->Result<(),MyErr>{
        self.create_response(&ctx.http, rply).await?;
        Ok(())
    }
    async fn get_msg(&self,ctx:&Context)->Result<Message,MyErr>{
        Ok(self.get_response(&ctx.http).await?)
    }
    async fn edit(&self,ctx:&Context,rlpy:EditInteractionResponse)->Result<(),MyErr>{
        self.edit_response(&ctx.http, rlpy).await?;
        Ok(())
    }
    fn user(&self)->User{
        self.user.clone()
    }
}
#[async_trait]
impl Mytrait for ModalInteraction{
    async fn err_response(&self,err:&ErrorLog<'_>,ephemeral:bool){
        if let Err(why) = self.create_response(&err.ctx.http, err.interaction_response(ephemeral,false)).await{
            MyErr::from(why).log_channel(err).await
        }
    }
    async fn err_defer(&self,err:&ErrorLog<'_>){
        if let Err(why) = self.edit_response(&err.ctx.http, err.defer_response(false)).await{
            MyErr::from(why).log_channel(err).await
        }
    }
    async fn defer_res(&self,err:&mut ErrorLog<'_>,on:&str,_ephemeral:bool){
        let res = self.defer(&err.ctx.http).await;
        if let Err(why) = res{
            let er = MyErr::from(why);
            err.change_error(er.get(),on.to_owned(), er.advice());
            err.log_error_channel(er.severity()).await;
        }
    }
    async fn response(&self,ctx:&Context,rply:CreateInteractionResponse)->Result<(),MyErr>{
        self.create_response(&ctx.http, rply).await?;
        Ok(())
    }
    async fn get_msg(&self,ctx:&Context)->Result<Message,MyErr>{
        Ok(self.get_response(&ctx.http).await?)
    }
    async fn edit(&self,ctx:&Context,rlpy:EditInteractionResponse)->Result<(),MyErr>{
        self.edit_response(&ctx.http, rlpy).await?;
        Ok(())
    }
    fn user(&self)->User{
        self.user.clone()
    }
}

#[async_trait]
impl Mytrait for ComponentInteraction{
    async fn err_response(&self,err:&ErrorLog<'_>,ephemeral:bool){
        if let Err(why) = self.create_response(&err.ctx.http, err.interaction_response(ephemeral,false)).await{
            MyErr::from(why).log_channel(err).await
        }
    }
    async fn err_defer(&self,err:&ErrorLog<'_>){
        if let Err(why) = self.edit_response(&err.ctx.http, err.defer_response(false)).await{
            MyErr::from(why).log_channel(err).await
        }
    }
    async fn defer_res(&self,err:&mut ErrorLog<'_>,on:&str,_ephemeral:bool){
        let res = self.defer(&err.ctx.http).await;
        if let Err(why) = res{
            let er = MyErr::from(why);
            err.change_error(er.get(),on.to_owned(), er.advice());
            err.log_error_channel(er.severity()).await;
        }
    }
    async fn response(&self,ctx:&Context,rply:CreateInteractionResponse)->Result<(),MyErr>{
        self.create_response(&ctx.http, rply).await?;
        Ok(())
    }
    async fn get_msg(&self,ctx:&Context)->Result<Message,MyErr>{
        Ok(self.get_response(&ctx.http).await?)
    }
    async fn edit(&self,ctx:&Context,rlpy:EditInteractionResponse)->Result<(),MyErr>{
        self.edit_response(&ctx.http, rlpy).await?;
        Ok(())
    }
    fn user(&self)->User{
        self.user.clone()
    }
}
