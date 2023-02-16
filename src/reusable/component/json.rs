use super::{Components,MyErr};
use std::path::PathBuf;
use serenity::all::*;


impl Components {
    pub fn get_att(cmd:&CommandInteraction)->Option<Attachment>{
        let resolved = &cmd.data.resolved;
        for i in &cmd.data.options{
            if let CommandDataOptionValue::Attachment(att)= &i.value{
                return Some(resolved.attachments.get(att)?.to_owned());
            }
        }
        None
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
