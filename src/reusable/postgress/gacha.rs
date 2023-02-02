use std::path::Path;

use sqlx::FromRow;

use crate::reusable::image_edit::gacha::GachaData;

use super::PgConn;

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
    pub async fn send_distrib(&self,pg:&GachaPg,data:&[GachaData],cid:i32)->Result<(),sqlx::Error>{
        sqlx::query("UPDATE discord set gacha=$1,pity=$2 where discord_id=$3").bind(pg.ticket)
        .bind(pg.pity).bind(&self.did).execute(&self.pool).await?;
        let folder = Path::new(".").join("binary");
        for i in data{
            let path = folder.join(&format!("{}.bin",&i.text));
            let data = tokio::fs::read(&path).await.unwrap();
            sqlx::query("INSERT into distribution (character_id,data,type,bot,event_name,description) 
Values ($1,$2,1,true,$3,$4)").bind(cid).bind(data).bind(&i.text).bind(&format!("~C05 Congratulation on Getting {}",&i.text)).execute(&self.pool).await?;
        }
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
