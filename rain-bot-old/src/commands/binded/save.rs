use serenity::builder::{CreateAttachment, CreateMessage};

use crate::{MyErr,Reg,Components, reusable::postgress::account::SaveData,Mytrait,Mybundle};

impl SaveData{
    fn get_attachment(&self)->Vec<CreateAttachment>{
        let mut x = Vec::new();
        if let Some(y)=&self.savedata{
            x.push(CreateAttachment::bytes(y.to_owned(), "savedata.bin"))
        }
        if let Some(y)=&self.partner{
            x.push(CreateAttachment::bytes(y.to_owned(), "partner.bin"))
        }
        if let Some(y)=&self.platebox{
            x.push(CreateAttachment::bytes(y.to_owned(), "platebox.bin"))
        }
        if let Some(y)=&self.skin_hist{
            x.push(CreateAttachment::bytes(y.to_owned(), "skin_hist.bin"))
        }
        if let Some(y)=&self.platedata{
            x.push(CreateAttachment::bytes(y.to_owned(), "platedata.bin"))
        }
        if let Some(y)=&self.otomoairou{
            x.push(CreateAttachment::bytes(y.to_owned(), "otomoairou.bin"))
        }
        if let Some(y)=&self.decomyset{
            x.push(CreateAttachment::bytes(y.to_owned(), "decomyset.bin"))
        }
        if let Some(y)=&self.hunternavi{
            x.push(CreateAttachment::bytes(y.to_owned(), "hunternavi.bin"))
        }
        if let Some(y)=&self.platemyset{
            x.push(CreateAttachment::bytes(y.to_owned(), "platemyset.bin"))
        }
        if let Some(y)=&self.rengokudata{
            x.push(CreateAttachment::bytes(y.to_owned(), "rengokudata.bin"))
        }
        if let Some(y)=&self.savemercenary{
            x.push(CreateAttachment::bytes(y.to_owned(), "savemercenary.bin"))
        }
        x
    }
}

#[hertz::hertz_combine_reg(300,false)]
async fn all<T:Mybundle>(bnd:&T,reg:&Reg<'_>)->Result<(),MyErr> {
    let data = reg.pg.send_save(reg.cid).await?;
    Components::response(bnd, "trying to Direct Message", true).await?;
    let user = bnd.user();
    let attachments = data.get_attachment();
    for i in attachments{
        user.direct_message(&bnd.ctx().http,CreateMessage::new().add_file(i)).await?;
    }
    Ok(())
}
