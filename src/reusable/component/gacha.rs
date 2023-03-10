use std::path::Path;
use rand::prelude::*;
use serde::{Serialize,Deserialize};
use crate::reusable::image_edit::gacha::{GachaData, GachaR};
use crate::reusable::postgress::gacha::GachaPg;
use super::super::bitwise::ItemCode;
use super::MyErr;


#[derive(Debug,Deserialize,Serialize)]
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
    pub async fn new()->Result<Gacha,MyErr>{
        let path = Path::new(".").join("static").join("gacha.json");
        Ok(serde_json::from_str(&tokio::fs::read_to_string(&path).await?)?)
    }
    pub async fn check(data:&[u8])->Result<(),MyErr>{
        serde_json::from_slice::<Self>(data)?;
        let path = Path::new(".").join("static").join("gacha.json");
        tokio::fs::write(path, data.to_vec()).await?;
        Ok(())
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
        if pity>=30{
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
            if pity>=30{
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
