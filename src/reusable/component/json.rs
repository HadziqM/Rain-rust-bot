use super::{Components,MyErr};
use std::path::PathBuf;
use serenity::all::*;


impl Components {
    pub fn get_att(cmd:&CommandInteraction)->Result<Attachment,MyErr>{
        let resolved:Vec<_>=cmd.data.resolved.attachments.iter().map(|x|x.1).collect();
        match resolved.first(){
            Some(x)=>{
                let idk = *x;
                Ok(idk.clone())
            }
            None=>Err(MyErr::Custom("cant get the attachment attachment".to_owned()))
        }
    }
    pub async fn download_check_and_save<T>(att:Attachment,path:&PathBuf,_tip:&T)->Result<(),MyErr>
        where for<'a> T:serde::Deserialize<'a>
    {
        let byte = att.download().await?;
        let utf8 = std::str::from_utf8(&byte)?.to_owned();
        serde_json::from_str::<T>(&utf8)?;
        tokio::fs::write(path, byte.clone()).await?;
        Ok(())
    }
}
