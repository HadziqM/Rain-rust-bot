use super::{MyErr,Mytrait};
use super::error::ErrorLog;
use serenity::all::*;

impl MyErr {
    pub(super) fn get(&self)->String{
        match self {
            MyErr::Custom(x)=>x.to_string(),
            MyErr::ByteWise(x)=>x.to_string(),
            MyErr::Utf8(x)=>x.to_string(),
            MyErr::Tokio(x)=>x.to_string(),
            MyErr::Serde(x)=>x.to_string(),
            MyErr::Serenity(x)=>x.to_string(),
            MyErr::Image(x)=>x.to_string()
        }
    }
    pub(super) fn advice(&self)->String{
        match self {
            MyErr::Custom(_)=>"Error message is writen by author themself, please read the message carafully or consult".to_string(),
            MyErr::ByteWise(_)=>"postgres connection (server database) error or data format error, you can report this or try again".to_string(),
            MyErr::Utf8(_)=>"parsing error while analizing file, are you sure you send a safe file?".to_string(),
            MyErr::Tokio(_)=>"file system error or paralel thread broken, report this!".to_string(),
            MyErr::Serde(_)=>"failed to operate with json data, that file might be broken or wrong format, carafully read the error message, it will tell which line in the file is wrong/invalid, then edit those file again".to_string(),
            MyErr::Serenity(_)=>"discord error, well discord unreasonably do this sometime, but rest assured, whatever you do, its already finished successfully, but if you find you missing something, you could report this".to_string(),
            MyErr::Image(_)=>"error on loading image or at image processing, you can report this to be investigated".to_owned()
        }
    }
    pub async fn log_channel(&self,err:&ErrorLog<'_>){
        err.log_error_channel().await
    }
    pub async fn log_msg(&self,msg:&Message,on:&'static str,err:&mut ErrorLog<'_>){
        err.change_error(self.get(), on, self.advice());
        if let Err(why) = msg.channel_id.send_message(&err.ctx.http, CreateMessage::new().embed(err.make_embed())).await{
            MyErr::from(why).log_channel(err).await
        }
    }
    pub async fn log_defer<T:Mytrait>(&self,cmd:&T,on:&'static str,err:&mut ErrorLog<'_>){
        err.change_error(self.get(), on, self.advice());
        if let MyErr::Serenity(_) = self{
            err.log_error_channel().await
        }else{
            cmd.err_defer(err).await;
        }
    }
    pub async fn log<T:Mytrait>(&self,cmd:&T,on:&'static str,ephemeral:bool,err:&mut ErrorLog<'_>){
        err.change_error(self.get(), on, self.advice());
        if let MyErr::Serenity(_) = self{
            err.log_error_channel().await
        }else{
            cmd.err_response(err, ephemeral).await;
        }
    }
}
