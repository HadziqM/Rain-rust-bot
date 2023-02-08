use sqlx::FromRow;
use super::PgConn;
use super::super::bitwise::{Bitwise,BitwiseError,ItemCode};

#[derive(FromRow,Debug)]
pub struct GachaPg{
    pub pity:i32,
    pub ticket:i32
}
impl<'a> PgConn<'a> {
    pub async fn get_pity(&self)->Result<GachaPg,sqlx::Error>{
        sqlx::query_as::<_,GachaPg>("SELECT gacha as ticket,pity From discord where discord_id=$1")
        .bind(&self.did).fetch_one(&self.pool).await
    }
    pub async fn send_distrib(&self,pg:&GachaPg,data:&[ItemCode],cid:i32)->Result<(),BitwiseError>{
        sqlx::query("UPDATE discord set gacha=$1,pity=$2 where discord_id=$3").bind(pg.ticket)
        .bind(pg.pity).bind(&self.did).execute(&self.pool).await?;
        let byte = Bitwise::new(data);
        if data.len() == 1{
            let text = match data.first().unwrap().text(){
                Some(x)=>x,
                None=>{return Err(BitwiseError::NoItem);}
            };
            sqlx::query("INSERT into distribution (character_id,data,type,bot,event_name,description) Values ($1,$2,1,true,$3,$4)").bind(cid).bind(byte.multiple_item()?).bind(&text).bind(&format!("~C05 Congratulation on Getting {}",&text)).execute(&self.pool).await?;
            return Ok(());
        }
        sqlx::query("INSERT into distribution (character_id,data,type,bot,event_name,description) Values ($1,$2,1,true,$3,$4)").bind(cid).bind(byte.multiple_item()?).bind("Multi Gacha Reward").bind("~C05 Sorry cant list all the reward in game").execute(&self.pool).await?;
        Ok(())
    }
    pub async fn market(&self,data:&ItemCode,cid:i32,price:Option<i32>)->Result<(),BitwiseError>{
        if let Some(x) = price{
            sqlx::query("UPDATE discord set bounty=bounty-$1 where discord_id=$2").bind(x)
            .bind(&self.did).execute(&self.pool).await?;
        }
        let array = [data.clone()];
        let byte = Bitwise::new(&array);
        sqlx::query("INSERT into distribution (character_id,data,type,bot,event_name,description) Values ($1,$2,1,true,$3,$4)").bind(cid).bind(byte.multiple_item()?).bind(data.text().unwrap()).bind("~C05 The item distributed by admin").execute(&self.pool).await?;
        Ok(())
    }
}

#[cfg(test)]
mod testing{
    use crate::reusable::config::get_config;

    use super::*;

    // #[tokio::test]
    // async fn get_pity() {
    //     let init = get_config().unwrap();
    //     let did = init.discord.author_id.to_string();
    //     let mut pg = PgConn::create(&init, did).await.unwrap();
    //     println!("{:?}",pg.get_pity().await.unwrap());
    //     pg.close().await;
    // }
    #[tokio::test]
    async fn send_distrib() {
        let init = get_config().unwrap();
        let did = init.discord.author_id.to_string();
        let mut pg = PgConn::create(&init, did).await.unwrap();
        let data = ItemCode { key: "0700".to_owned(), count: 1, types: 7 };
        pg.market(&data, pg.get_char_id().await.unwrap().0, Some(1)).await.unwrap();
        pg.close().await;
    }
    #[tokio::test]
    async fn single() {
        let init = get_config().unwrap();
        let did = init.discord.author_id.to_string();
        let mut pg = PgConn::create(&init, did).await.unwrap();
        let data = [ItemCode { key: "0700".to_owned(), count: 1, types: 7 }];
        let gac = GachaPg{pity:30,ticket:20};
        pg.send_distrib(&gac, &data, pg.get_char_id().await.unwrap().0).await.unwrap();
        pg.close().await;
    }
    #[tokio::test]
    async fn multiple() {
        let init = get_config().unwrap();
        let did = init.discord.author_id.to_string();
        let mut pg = PgConn::create(&init, did).await.unwrap();
        let data = [ItemCode { key: "0700".to_owned(), count: 1, types: 7 },
        ItemCode { key: "0700".to_owned(), count: 20, types: 7 }];
        let gac = GachaPg{pity:30,ticket:20};
        pg.send_distrib(&gac, &data, pg.get_char_id().await.unwrap().0).await.unwrap();
        pg.close().await;
    }
}
