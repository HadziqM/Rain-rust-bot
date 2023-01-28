use serenity::{prelude::Context, model::prelude::{interaction::{application_command::ApplicationCommandInteraction, InteractionResponseType, message_component::MessageComponentInteraction}, AttachmentType}};

use crate::{Init,Register, reusable::postgress::account::SaveData};
use std::borrow::Cow;
impl SaveData{
    fn get_attachment(&self)->Vec<AttachmentType>{
        let mut x = Vec::new();
        if let Some(y)=&self.savedata{
            x.push(AttachmentType::Bytes { data:Cow::from(y.clone()),
                filename: "savedata.bin".to_string()});
        }
        if let Some(y)=&self.partner{
            x.push(AttachmentType::Bytes { data:Cow::from(y.clone()),
                filename: "partner.bin".to_string()});
        }
        if let Some(y)=&self.platebox{
            x.push(AttachmentType::Bytes { data:Cow::from(y.clone()),
                filename: "platebox.bin".to_string()});
        }
        if let Some(y)=&self.skin_hist{
            x.push(AttachmentType::Bytes { data:Cow::from(y.clone()),
                filename: "skin_hist.bin".to_string()});
        }
        if let Some(y)=&self.platedata{
            x.push(AttachmentType::Bytes { data:Cow::from(y.clone()),
                filename: "platedata.bin".to_string()});
        }
        if let Some(y)=&self.otomoairou{
            x.push(AttachmentType::Bytes { data:Cow::from(y.clone()),
                filename: "otomoairou.bin".to_string()});
        }
        if let Some(y)=&self.decomyset{
            x.push(AttachmentType::Bytes { data:Cow::from(y.clone()),
                filename: "decomyset.bin".to_string()});
        }
        if let Some(y)=&self.hunternavi{
            x.push(AttachmentType::Bytes { data:Cow::from(y.clone()),
                filename: "hunternavi.bin".to_string()});
        }
        if let Some(y)=&self.platemyset{
            x.push(AttachmentType::Bytes { data:Cow::from(y.clone()),
                filename: "platemyset.bin".to_string()});
        }
        if let Some(y)=&self.rengokudata{
            x.push(AttachmentType::Bytes { data:Cow::from(y.clone()),
                filename: "rengokudata.bin".to_string()});
        }
        if let Some(y)=&self.savemercenary{
            x.push(AttachmentType::Bytes { data:Cow::from(y.clone()),
                filename: "savemercenary.bin".to_string()});
        }
        x
    }
}

pub async fn run(ctx:&Context,cmd:&ApplicationCommandInteraction,init:&Init){
    let mut reg = match Register::default(ctx, cmd, init, "dm_save command", false).await{
        Some(r)=>r,
        None=>{return ;}
    };
    match reg.pg.send_save(reg.cid).await{
        Ok(dt)=>{
            if let Err(why)=cmd.user.direct_message(&ctx.http, |m|{
                m.content("your save").add_files(dt.get_attachment())
            }).await{
                reg.error.change_error(why.to_string(), "send dm_save", "maybe you need to enable dm".to_string())
            }
            if let Err(why)=cmd.create_interaction_response(&ctx.http, |n|{
                n.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|m|m.content("tying to dm"))
            }).await{
                reg.error.discord_error(why.to_string(), "dm save").await;
            }
        }
        Err(why)=>{
            reg.error.pgcon_error(why.to_string(), "getting save", cmd).await;
        }
    }
    reg.pg.close().await;
}
pub async fn run_button(ctx:&Context,cmd:&MessageComponentInteraction,init:&Init){
    let mut reg = match Register::default_button(ctx, cmd, init, "dm_save command").await{
        Some(r)=>r,
        None=>{return ;}
    };
    match reg.pg.send_save(reg.cid).await{
        Ok(dt)=>{
            if let Err(why)=cmd.user.direct_message(&ctx.http, |m|{
                m.content("your save").add_files(dt.get_attachment())
            }).await{
                reg.error.change_error(why.to_string(), "send dm_save", "maybe you need to enable dm".to_string())
            }
            if let Err(why)=cmd.create_interaction_response(&ctx.http, |n|{
                n.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|m|m.content("tying to dm"))
            }).await{
                reg.error.discord_error(why.to_string(), "dm save").await;
            }
        }
        Err(why)=>{
            reg.error.pgcon_error_button(why.to_string(), "getting save", cmd).await;
        }
    }
    reg.pg.close().await;
}
