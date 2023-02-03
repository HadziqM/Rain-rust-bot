use serenity::{builder::{CreateAttachment, CreateMessage, CreateInteractionResponse, CreateInteractionResponseMessage}, prelude::Context, all::{CommandInteraction, ComponentInteraction}};
use crate::{Init,Register,Components, reusable::postgress::account::SaveData};
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

pub async fn run(ctx:&Context,cmd:&CommandInteraction,init:&Init){
    let mut reg = match Register::default(ctx, cmd, init, "dm_save command", false).await{
        Some(r)=>r,
        None=>{return ;}
    };
    match reg.pg.send_save(reg.cid).await{
        Ok(dt)=>{
            if let Err(why)=cmd.create_response(&ctx.http,CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content("trying to dm")
                    )).await{
                reg.error.discord_error(why.to_string(), "dm save").await;
            }
            if let Err(why)=cmd.user.direct_message(&ctx.http,CreateMessage::new()
                .content("your save").add_files(dt.get_attachment())).await{
                reg.error.change_error(why.to_string(), "send dm_save", "maybe you need to enable dm".to_string());
                reg.error.log_error_channel().await;
            }
        }
        Err(why)=>{
            reg.error.pgcon_error(why.to_string(), "getting save", cmd).await;
        }
    }
    reg.pg.close().await;
}
pub async fn run_button(ctx:&Context,cmd:&ComponentInteraction,init:&Init){
    let mut reg = match Register::default_button(ctx, cmd, init, "dm_save command").await{
        Some(r)=>r,
        None=>{return ;}
    };
    match reg.pg.send_save(reg.cid).await{
        Ok(dt)=>{
            if let Err(why)=cmd.user.direct_message(&ctx.http,CreateMessage::new().content("your save")
                .add_files(dt.get_attachment())).await{
                reg.error.change_error(why.to_string(), "send dm_save", "maybe you need to enable dm".to_string())
            }
            if let Err(why)=cmd.create_response(&ctx.http,Components::interaction_response("truing to dm_save", true)).await{
                reg.error.discord_error(why.to_string(), "dm save").await;
            }
        }
        Err(why)=>{
            reg.error.pgcon_error_button(why.to_string(), "getting save", cmd).await;
        }
    }
    reg.pg.close().await;
}
