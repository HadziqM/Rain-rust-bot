use sqlx::{FromRow, Row};
use crate::material::ItemPedia;
use super::PgConn;
use super::super::bitwise::{Bitwise,BitwiseError,ItemCode};
use super::card::Event;

#[derive(FromRow,Debug)]
pub struct GachaPg{
    pub pity:i32,
    pub ticket:i32
}

impl<'a> PgConn<'a> {
    pub async fn get_coin(&self)->Result<i32,BitwiseError>{
        Ok(sqlx::query("SELECT bounty From discord where discord_id=$1")
        .bind(&self.did).fetch_one(&self.pool).await?.get("bounty"))
    }
    pub async fn get_pity(&self)->Result<GachaPg,BitwiseError>{
        Ok(sqlx::query_as::<_,GachaPg>("SELECT gacha as ticket,pity From discord where discord_id=$1")
        .bind(&self.did).fetch_one(&self.pool).await?)
    }
    pub async fn buy_ticket(&self,ticket:i32)->Result<(),BitwiseError>{
        sqlx::query("update discord set gacha=gacha+$1 where discord_id=$2").bind(ticket).bind(&self.did).execute(&self.pool).await?;
        Ok(())
    }
    pub async fn ticket_all(&self,ticket:i32)->Result<(),BitwiseError>{
        sqlx::query("update discord set gacha=gacha+$1 where ticket is not null").bind(ticket).bind(&self.did).execute(&self.pool).await?;
        Ok(())
    }
    pub async fn send_item(&self,data:&[ItemCode],cid:i32,name:&str,desc:&str)->Result<(),BitwiseError>{
        let byte = Bitwise::new(data);
        sqlx::query("INSERT into distribution (character_id,data,type,bot,event_name,description) Values ($1,$2,1,true,$3,$4)").bind(cid).bind(byte.multiple_item()?).bind(name).bind(&format!("~C05 {}",desc)).execute(&self.pool).await?;
        Ok(())
    }
    pub async fn send_distrib(&self,pg:&GachaPg,data:&[ItemCode],cid:i32,pedia:&ItemPedia)->Result<(),BitwiseError>{
        sqlx::query("UPDATE discord set gacha=$1,pity=$2 where discord_id=$3").bind(pg.ticket)
        .bind(pg.pity).bind(&self.did).execute(&self.pool).await?;
        if data.len() == 1{
            let text = match data.first().unwrap().text(pedia){
                Some(x)=>x,
                None=>{return Err(BitwiseError::NoItem);}
            };
            self.send_item(data, cid, &text, &format!("congratulation on getting {}",&text)).await?;
            return Ok(());
        }
        self.send_item(data, cid, "Multi Gacha Rewards", "cant list all reward in description").await?;
        Ok(())
    }
    pub async fn bounty_transaction(&self,price:i32)->Result<(),BitwiseError>{
        sqlx::query("UPDATE discord set bounty=bounty-$1 where discord_id=$2").bind(price)
        .bind(&self.did).execute(&self.pool).await?;
        Ok(())
    }
    pub async fn bounty_event(&self,event:&Event)->Result<(),BitwiseError>{
        sqlx::query("UPDATE discord set bounty=$1,gacha=$2,gold=$3,silver=$4,bronze=$5,latest_bounty=$6
            ,latest_bounty_time=$6,title=$7 where discord_id=$8")
            .bind(event.bounty).bind(event.gacha).bind(event.gold).bind(event.silver).bind(event.bronze)
            .bind(&event.latest_bounty).bind(event.latest_bounty_time).bind(event.title).bind(&self.did)
            .execute(&self.pool).await?;
        Ok(())
    }
    pub async fn bounty_all(&self,gift:i32)->Result<(),BitwiseError>{
        sqlx::query("UPDATE discord set bounty=bounty+$1 where bounty is not null").bind(gift)
        .bind(&self.did).execute(&self.pool).await?;
        Ok(())
    }
    pub async fn market(&self,data:&ItemCode,cid:i32,price:Option<i32>,pedia:&ItemPedia)->Result<(),BitwiseError>{
        if let Some(x) = price{
            self.bounty_transaction(x).await?;
        }
        let array = [data.clone()];
        let byte = Bitwise::new(&array);
        sqlx::query("INSERT into distribution (character_id,data,type,bot,event_name,description) Values ($1,$2,1,true,$3,$4)").bind(cid).bind(byte.multiple_item()?).bind(data.text(pedia).unwrap()).bind("~C05 The item distributed by admin").execute(&self.pool).await?;
        Ok(())
    }
    pub async fn market_user(&self,data:&ItemCode,cid:i32,price:u32,pedia:&ItemPedia)->Result<(),BitwiseError>{
        self.bounty_transaction(price as i32).await?;
        let array = [data.clone()];
        let byte = Bitwise::new(&array);
        sqlx::query("INSERT into distribution (character_id,data,type,bot,event_name,description) Values ($1,$2,1,true,$3,$4)").bind(cid).bind(byte.multiple_item()?).bind(data.text(pedia).unwrap()).bind("~C05 The market transaction delivery").execute(&self.pool).await?;
        Ok(())
    }
}

// #[cfg(test)]
// mod testing{
//     use crate::reusable::config::get_config;
//
//     use super::*;
//
//     // #[tokio::test]
//     // async fn get_pity() {
//     //     let init = get_config().unwrap();
//     //     let did = init.discord.author_id.to_string();
//     //     let mut pg = PgConn::create(&init, did).await.unwrap();
//     //     println!("{:?}",pg.get_pity().await.unwrap());
//     //     pg.close().await;
//     // }
//     #[tokio::test]
//     async fn send_distrib() {
//         let init = get_config().unwrap();
//         let did = init.discord.author_id.to_string();
//         let mut pg = PgConn::create(&init, did).await.unwrap();
//         let data = ItemCode { key: "0700".to_owned(), count: 1, types: 7 };
//         pg.market(&data, 843, Some(1)).await.unwrap();
//         pg.close().await;
//     }
//     #[tokio::test]
//     async fn single() {
//         let init = get_config().unwrap();
//         let did = init.discord.author_id.to_string();
//         let mut pg = PgConn::create(&init, did).await.unwrap();
//         let data = [ItemCode { key: "0700".to_owned(), count: 1, types: 7 }];
//         let gac = GachaPg{pity:30,ticket:20};
//         pg.send_distrib(&gac, &data,843).await.unwrap();
//         pg.close().await;
//     }
//     #[tokio::test]
//     async fn multiple() {
//         let init = get_config().unwrap();
//         let did = init.discord.author_id.to_string();
//         let mut pg = PgConn::create(&init, did).await.unwrap();
//         let data = [ItemCode { key: "0700".to_owned(), count: 1, types: 7 },
//         ItemCode { key: "0700".to_owned(), count: 20, types: 7 }];
//         let gac = GachaPg{pity:30,ticket:20};
//         pg.send_distrib(&gac, &data,843).await.unwrap();
//         pg.close().await;
//     }
// }
