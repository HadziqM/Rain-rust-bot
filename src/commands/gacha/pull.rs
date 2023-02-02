use std::borrow::Cow;
use std::path::Path;

use serde::Deserialize;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::AttachmentType;
use serenity::model::user::User;
use serenity::{prelude::Context, model::prelude::interaction::application_command::ApplicationCommandInteraction};
use crate::reusable::image_edit::gacha::{GachaData, GachaR, GachaImage};
use crate::reusable::postgress::gacha::GachaPg;
use crate::{Init,Register};
use rand::prelude::*;

#[derive(Debug,Deserialize)]
struct Gacha {
    ur: Vec<String>,
    ssr1: Vec<String>,
    ssr2: Vec<String>,
    sr1: Vec<String>,
    sr2:Vec<String>,
    sr3: Vec<String>,
    r1: Vec<String>,
    r2: Vec<String>,
    download: Vec<Download>
}
#[derive(Deserialize,Debug)]
struct Download{
    name:String,
    distribution:i32
}

impl Gacha{
    async fn new()->Gacha{
        let path = Path::new(".").join("gacha").join("gacha.json");
        serde_json::from_str(&tokio::fs::read_to_string(&path).await.unwrap()).unwrap()
    }
    fn pull(&self)->GachaData{
        let  mut thread = rand::thread_rng();
        let rng:f32 = thread.gen();
        //define ur = 0.1%
        if rng<=0.001{
            let mut ur = self.ur.clone();
            ur.shuffle(&mut thread);
            return GachaData{text:ur[0].to_owned(),result:GachaR::UR};
        //define ssr1 = 1%~0.1% = 0.9%
        }else if rng<=0.01{
            let mut ssr1 = self.ssr1.clone();
            ssr1.shuffle(&mut thread);
            return GachaData{text:ssr1[0].to_owned(),result:GachaR::SSR};
        //define ssr2 = 3%~1% = 2%
        }else if rng<=0.03{
            let mut ssr2 = self.ssr2.clone();
            ssr2.shuffle(&mut thread);
            return GachaData{text:ssr2[0].to_owned(),result:GachaR::SSR};
        //define sr1 = 8%~3% = 5%
        }else if rng<=0.08{
            let mut sr1 = self.sr1.clone();
            sr1.shuffle(&mut thread);
            return GachaData{text:sr1[0].to_owned(),result:GachaR::SR};
        //define sr2 = 8%~18% = 10%
        }else if rng<=0.18{
            let mut sr2 = self.sr2.clone();
            sr2.shuffle(&mut thread);
            return GachaData{text:sr2[0].to_owned(),result:GachaR::SR};
        //define sr3 = 18%~33% = 15%
        }else if rng<=0.33{
            let mut sr3 = self.sr3.clone();
            sr3.shuffle(&mut thread);
            return GachaData{text:sr3[0].to_owned(),result:GachaR::SR};
        //define r1 = 33%~63% = 30%
        }else if rng<=0.63{
            let mut r1 = self.r1.clone();
            r1.shuffle(&mut thread);
            return GachaData{text:r1[0].to_owned(),result:GachaR::R};
        }
        //define r2 = 63%~100% = 37%
        let mut r2 = self.r2.clone();
        r2.shuffle(&mut thread);
        GachaData{text:r2[0].to_owned(),result:GachaR::R}
    }
    fn guaranteed(&self)->GachaData{
        let  mut thread = rand::thread_rng();
        let rng:f32 = thread.gen();
        //define UR = 20%
        if rng<=0.2{
            let mut ur = self.ur.clone();
            ur.shuffle(&mut thread);
            return GachaData{text:ur[0].to_owned(),result:GachaR::UR};
        //define ssr1 = 50%~20% = 30%
        }else if rng<=0.5{
            let mut ssr1 = self.ssr1.clone();
            ssr1.shuffle(&mut thread);
            return GachaData{text:ssr1[0].to_owned(),result:GachaR::SSR};
        //define ssr2 = 50%~100% = 50%
        }
        let mut ssr2 = self.ssr2.clone();
        ssr2.shuffle(&mut thread);
        GachaData{text:ssr2[0].to_owned(),result:GachaR::SSR}
    }
    pub fn single_pull(&self,data:&GachaPg)->(GachaPg,GachaData){
        let pity = data.pity+1;
        let ticket = data.ticket-10;
        if pity==50{
            return (GachaPg{pity:0,ticket},self.guaranteed());
        }
        (GachaPg{pity,ticket},self.pull())
    }
    pub fn multi_pull(&self,data:&GachaPg)->(GachaPg,Vec<GachaData>){
        let mut pity = data.pity;
        let ticket = data.ticket-110;
        let mut res = Vec::new();
        for _ in 0..12{
            pity += 1;
            if pity==50{
                res.push(self.guaranteed());
                pity = 0
            }else{
                res.push(self.pull());
            }
        }
        return (GachaPg{pity,ticket},res);
    }
}
fn create_embed(user:&User,pg:&GachaPg)->CreateEmbed{
    let mut emb = CreateEmbed::default();
    emb.title("Mhfz Gacha Result").description(format!("Pity Count: {}\nTicket Remaining: {}",pg.pity,pg.ticket)).author(|a|a.name(user.name).icon_url(user.face())).image("attachment://gacha.jpg");
    emb
}
pub async fn run(ctx:&Context,cmd:&ApplicationCommandInteraction,init:&Init,multi:bool){
    let mut reg =match Register::default(ctx, cmd, init, "single pull", false).await{
        Some(x)=>x,
        None=>{return;}
    };
    if let Err(why)=cmd.defer(&ctx.http).await{
        reg.error.discord_error(why.to_string(), "defering pull slash").await;
    }
    let data = match reg.pg.get_pity().await{
        Ok(x)=>x,
        Err(why)=>{
            reg.error.pgcon_error_defer(why.to_string(), "getting gacha data", cmd).await;
            return reg.pg.close().await;
        }
    };
    let gacha = Gacha::new().await;
    let image = match GachaImage::new(&cmd.user.static_avatar_url()
        .unwrap_or(cmd.user.default_avatar_url())).await{
            Ok(x)=>x,
            Err(why)=>{
                reg.error.change_error(why.to_string(), "image init", "please report to admin".to_string());
                reg.error.log_slash(cmd, false).await;
                return reg.pg.close().await;
            }
        };
    let raw;
    let g_pg;
    let g_data;
    let cost = ||{
        if multi{
            return 110;
        }
        10
    };
    if data.ticket<cost(){
            reg.error.change_error("not enough ticket".to_string(),"multi pull",format!("you only have {} ticket and its need {} ticket to pull, so you need to collect {} more",data.ticket,cost(),cost()-data.ticket));
            reg.error.log_slash_defer(cmd, false).await;
            return reg.pg.close().await;
    }
    if !multi{
        let result = gacha.single_pull(&data);
        g_pg = result.0;
        raw =image.single_pull(&result.1).await;
        g_data = vec![result.1];
    }else{
        let result = gacha.multi_pull(&data);
        g_pg=result.0;
        raw = image.multi_pull(result.1.clone()).await;
        g_data = result.1;
    }
    match raw{
        Ok(x)=>{
            if let Err(why)=reg.pg.send_distrib(&g_pg, g_data.as_slice(), reg.cid).await{
                reg.error.pgcon_error_defer(why.to_string(), "sending distribution", cmd).await;
                return reg.pg.close().await;
            }
            let att = AttachmentType::Bytes { data:Cow::from(x), filename:"gacha.jpg".to_string() };
            let embed = create_embed(&cmd.user, &g_pg);
            if let Err(why)=cmd.edit_original_interaction_response(&ctx.http, |m|{
                m.add_embed(embed)
            }).await{
                reg.error.discord_error(why.to_string(), "sending gacha result").await;
                return reg.pg.close().await;
            }
        }
        Err(why)=>{
            reg.error.change_error(why.to_string(), "edit image", "please report".to_string());
            reg.error.log_slash_defer(cmd, true).await;
            return reg.pg.close().await;
        }
    };
}
