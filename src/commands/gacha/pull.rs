use std::path::Path;
use serde::Deserialize;
use serenity::builder::CreateEmbed;
use serenity::all::*;
use crate::reusable::image_edit::gacha::{GachaData, GachaR, GachaImage};
use crate::reusable::bitwise::ItemCode;
use crate::reusable::postgress::gacha::GachaPg;
use crate::{Reg,SlashBundle,MyErr,Components};
use crate::reusable::utils::Color;
use rand::prelude::*;

#[derive(Debug,Deserialize)]
pub struct Gacha {
    ur: Vec<ItemCode>,
    ssr1: Vec<ItemCode>,
    ssr2: Vec<ItemCode>,
    sr1: Vec<ItemCode>,
    sr2:Vec<ItemCode>,
    sr3: Vec<ItemCode>,
    r1: Vec<ItemCode>,
    r2: Vec<ItemCode>,
}
impl Default for Gacha {
    fn default() -> Self {
        Gacha { ur: Vec::new(), ssr1: Vec::new(), ssr2: Vec::new(), sr1: Vec::new()
            , sr2: Vec::new(), sr3: Vec::new(), r1: Vec::new(), r2: Vec::new() }
    }
}

impl Gacha{
    async fn new()->Result<Gacha,MyErr>{
        let path = Path::new(".").join("static").join("gacha.json");
        Ok(serde_json::from_str(&tokio::fs::read_to_string(&path).await?)?)
    }
    fn pull(&self)->GachaData{
        let  mut thread = rand::thread_rng();
        let rng:f32 = thread.gen();
        //define ur = 0.1%
        if rng<=0.001{
            let mut ur = self.ur.clone();
            ur.shuffle(&mut thread);
            return GachaData{code:ur[0].to_owned(),result:GachaR::UR};
        //define ssr1 = 1%~0.1% = 0.9%
        }else if rng<=0.01{
            let mut ssr1 = self.ssr1.clone();
            ssr1.shuffle(&mut thread);
            return GachaData{code:ssr1[0].to_owned(),result:GachaR::SSR};
        //define ssr2 = 3%~1% = 2%
        }else if rng<=0.03{
            let mut ssr2 = self.ssr2.clone();
            ssr2.shuffle(&mut thread);
            return GachaData{code:ssr2[0].to_owned(),result:GachaR::SSR};
        //define sr1 = 8%~3% = 5%
        }else if rng<=0.08{
            let mut sr1 = self.sr1.clone();
            sr1.shuffle(&mut thread);
            return GachaData{code:sr1[0].to_owned(),result:GachaR::SR};
        //define sr2 = 8%~18% = 10%
        }else if rng<=0.18{
            let mut sr2 = self.sr2.clone();
            sr2.shuffle(&mut thread);
            return GachaData{code:sr2[0].to_owned(),result:GachaR::SR};
        //define sr3 = 18%~33% = 15%
        }else if rng<=0.33{
            let mut sr3 = self.sr3.clone();
            sr3.shuffle(&mut thread);
            return GachaData{code:sr3[0].to_owned(),result:GachaR::SR};
        //define r1 = 33%~63% = 30%
        }else if rng<=0.63{
            let mut r1 = self.r1.clone();
            r1.shuffle(&mut thread);
            return GachaData{code:r1[0].to_owned(),result:GachaR::R};
        }
        //define r2 = 63%~100% = 37%
        let mut r2 = self.r2.clone();
        r2.shuffle(&mut thread);
        GachaData{code:r2[0].to_owned(),result:GachaR::R}
    }
    fn guaranteed(&self)->GachaData{
        let  mut thread = rand::thread_rng();
        let rng:f32 = thread.gen();
        //define UR = 20%
        if rng<=0.2{
            let mut ur = self.ur.clone();
            ur.shuffle(&mut thread);
            return GachaData{code:ur[0].to_owned(),result:GachaR::UR};
        //define ssr1 = 50%~20% = 30%
        }else if rng<=0.5{
            let mut ssr1 = self.ssr1.clone();
            ssr1.shuffle(&mut thread);
            return GachaData{code:ssr1[0].to_owned(),result:GachaR::SSR};
        //define ssr2 = 50%~100% = 50%
        }
        let mut ssr2 = self.ssr2.clone();
        ssr2.shuffle(&mut thread);
        GachaData{code:ssr2[0].to_owned(),result:GachaR::SSR}
    }
    pub fn single_pull(&self,data:&GachaPg)->(GachaPg,GachaData){
        let pity = data.pity+1;
        let ticket = data.ticket-10;
        if pity==30{
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
                let pull = self.pull();
                if pull.result == GachaR::SSR || pull.result == GachaR::UR{
                    pity = 0
                }
                res.push(pull);
            }
        }
        return (GachaPg{pity,ticket},res);
    }
}
fn create_embed(user:&User,pg:&GachaPg)->CreateEmbed{
    CreateEmbed::new().title("Mhfz Gacha Result").description(format!("Pity Count: {}\nTicket Remaining: {}",pg.pity,pg.ticket))
        .author(CreateEmbedAuthor::new(&user.name).icon_url(user.face())).image("attachment://gacha.jpg").color(Color::Random.throw())
}

use crate::{Mybundle,Mytrait};
#[hertz::hertz_slash_reg(60,true)]
async fn slash(bnd:&SlashBundle<'_>,mut reg:Reg<'_>)->Result<(),MyErr>{
    let mut multi = false;
    for i in &bnd.cmd.data.options{
        if let CommandDataOptionValue::SubCommand(_) = &i.value{
            if i.name == "multi"{
                multi = true;
            }
        }
    }
    let data = reg.pg.get_pity().await?;
    let gacha = Gacha::new().await?;
    let image = GachaImage::new(&bnd.cmd.user.static_avatar_url().unwrap_or(bnd.cmd.user.default_avatar_url())).await?;
    let raw;
    let g_pg;
    let g_data;
    let cost = ||{if multi{return 110;}10};
    if data.ticket<cost(){
            reg.pg.close().await;
            return Err(MyErr::Custom(format!("insufficient ticket, you only have {} ticket and its need {} ticket to pull, so you need to collect {} more",data.ticket,cost(),cost()-data.ticket)));
    }
    if !multi{
        let result = gacha.single_pull(&data);
        g_pg = result.0;
        raw =image.single_pull(&result.1,bnd.pedia,bnd.image).await?;
        g_data = vec![result.1];
    }else{
        let result = gacha.multi_pull(&data);
        g_pg=result.0;
        raw = image.multi_pull(result.1.clone(),bnd.pedia,bnd.image).await?;
        g_data = result.1;
    }
    let code:Vec<_> = g_data.iter().map(|e|e.code.to_owned()).collect();
    reg.pg.send_distrib(&g_pg,&code, reg.cid, bnd.pedia).await?;
    let att = CreateAttachment::bytes(raw, "gacha.jpg");
    let embed = create_embed(&bnd.cmd.user, &g_pg);
    let content = EditInteractionResponse::new()
        .new_attachment(att).embed(embed);
    Components::edit_adv(bnd, content).await?;
    reg.pg.close().await;
    Ok(())
}
