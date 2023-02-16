use super::MyErr;
use super::error::ErrorLog;
use serenity::all::*;

impl MyErr {
    fn get(&self)->String{
        match self {
            MyErr::Custom(x)=>x.to_string(),
            MyErr::ByteWise(x)=>x.to_string(),
            MyErr::Utf8(x)=>x.to_string(),
            MyErr::Tokio(x)=>x.to_string(),
            MyErr::Serde(x)=>x.to_string(),
            MyErr::Serenity(x)=>x.to_string(),
        }
    }
    fn advice(&self)->String{
        match self {
            MyErr::Custom(_)=>"Error message is writen by author theirself, please read the message carafully or consult".to_string(),
            MyErr::ByteWise(_)=>"postgres connection (server database) error or data format error, you can report this or try again".to_string(),
            MyErr::Utf8(_)=>"parsing error while analizing file, are you sure you send a safe file?".to_string(),
            MyErr::Tokio(_)=>"file system error or paralel thread broken, report this!".to_string(),
            MyErr::Serde(_)=>"failed to operate with json data, some file might be broken, or wrong json configuration, please check".to_string(),
            MyErr::Serenity(_)=>"discord error, well discord unreasonably do this sometime, but rest assured, whatever you do, its already finished successfully, but if you find you missing something, you could report this".to_string(),
        }
    }
    pub async fn log_slash(&self,cmd:&CommandInteraction,on:&'static str,ephemeral:bool,err:&mut ErrorLog<'_>){
        err.change_error(self.get(), on, self.advice());
        if let MyErr::Serenity(_) = self{
            err.log_error_channel().await
        }else{
            err.log_slash(cmd, ephemeral).await;
        }
    }
    pub async fn log_defer(&self,cmd:&CommandInteraction,on:&'static str,ephemeral:bool,err:&mut ErrorLog<'_>){
        err.change_error(self.get(), on, self.advice());
        if let MyErr::Serenity(_) = self{
            err.log_error_channel().await
        }else{
            err.log_slash_defer(cmd, ephemeral).await;
        }
    }
    pub async fn log_button(&self,cmd:&ComponentInteraction,on:&'static str,ephemeral:bool,err:&mut ErrorLog<'_>){
        err.change_error(self.get(), on, self.advice());
        if let MyErr::Serenity(_) = self{
            err.log_error_channel().await
        }else{
            err.log_button(cmd, ephemeral).await;
        }
    }
    pub async fn log_modal(&self,cmd:&ModalInteraction,on:&'static str,ephemeral:bool,err:&mut ErrorLog<'_>){
        err.change_error(self.get(), on, self.advice());
        if let MyErr::Serenity(_) = self{
            err.log_error_channel().await
        }else{
            err.log_modal(cmd, ephemeral).await;
        }
    }
}
