use std::collections::HashMap;

use sqlx::FromRow;

use crate::reusable::image_edit::gacha::GachaData;

use super::PgConn;
use crate::material::items::Items;
use super::super::bitwise::{Bitwise,BitwiseError};

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
    pub async fn send_distrib(&self,pg:&GachaPg,data:&[GachaData],cid:i32)->Result<(),BitwiseError>{
        sqlx::query("UPDATE discord set gacha=$1,pity=$2 where discord_id=$3").bind(pg.ticket)
        .bind(pg.pity).bind(&self.did).execute(&self.pool).await?;
        let item = Items::default();
        let text = |i:&GachaData|{format!("{}x{}",item.item.get(i.code.key.as_str()).unwrap(),i.code.count)};
        if data.len() == 1{
            let byte = Bitwise::single_item(&data[0].code.key, data[0].code.count)?;
            sqlx::query("INSERT into distribution (character_id,data,type,bot,event_name,description) Values ($1,$2,1,true,$3,$4)").bind(cid).bind(byte).bind(&text(&data[0])).bind(&format!("~C05 Congratulation on Getting {}",&text(&data[0]))).execute(&self.pool).await?;
            return Ok(());
        }
        let buffer:HashMap<&str,u16> = data.iter().map(|m|(m.code.key.as_str(),m.code.count)).collect::<HashMap<_,_>>();
        let byte = Bitwise::multiple_item(buffer)?;
        sqlx::query("INSERT into distribution (character_id,data,type,bot,event_name,description) Values ($1,$2,1,true,$3,$4)").bind(cid).bind(byte).bind("Multi Gacha Reward").bind("~C05 Sorry cant list all the reward in game").execute(&self.pool).await?;
        Ok(())
    }
}

#[cfg(test)]
mod testing{
    use crate::reusable::config::get_config;

    use super::*;

    #[tokio::test]
    async fn get_pity() {
        let init = get_config().unwrap();
        let did = init.discord.author_id.to_string();
        let mut pg = PgConn::create(&init, did).await.unwrap();
        println!("{:?}",pg.get_pity().await.unwrap());
        pg.close().await;
    }
}
