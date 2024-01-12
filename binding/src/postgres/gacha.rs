use super::super::bitwise::{Bitwise, ItemCode};
use super::card::Event;
use super::{Db, PgCustomError};
use material::ItemPedia;
use sqlx::{FromRow, Row};

#[derive(FromRow, Debug)]
pub struct GachaPg {
    pub pity: i32,
    pub ticket: i32,
}

impl Db {
    pub async fn get_coin(&self, did: &str) -> Result<i32, PgCustomError> {
        Ok(
            sqlx::query("SELECT bounty From discord where discord_id=$1")
                .bind(did)
                .fetch_one(&**self)
                .await?
                .get("bounty"),
        )
    }
    pub async fn get_pity(&self, did: &str) -> Result<GachaPg, PgCustomError> {
        Ok(sqlx::query_as::<_, GachaPg>(
            "SELECT gacha as ticket,pity From discord where discord_id=$1",
        )
        .bind(did)
        .fetch_one(&**self)
        .await?)
    }
    pub async fn buy_ticket(&self, did: &str, ticket: i32) -> Result<(), PgCustomError> {
        sqlx::query("update discord set gacha=gacha+$1 where discord_id=$2")
            .bind(ticket)
            .bind(did)
            .execute(&**self)
            .await?;
        Ok(())
    }
    pub async fn ticket_all(&self, did: &str, ticket: i32) -> Result<(), PgCustomError> {
        sqlx::query("update discord set gacha=gacha+$1 where gacha is not null")
            .bind(ticket)
            .bind(did)
            .execute(&**self)
            .await?;
        Ok(())
    }
    pub async fn send_item(
        &self,
        data: &[ItemCode],
        cid: i32,
        name: &str,
        desc: &str,
    ) -> Result<(), PgCustomError> {
        let byte = Bitwise::new(data);
        sqlx::query("INSERT into distribution (character_id,data,type,bot,event_name,description) Values ($1,$2,1,true,$3,$4)").bind(cid).bind(byte.multiple_item().unwrap()).bind(name).bind(&format!("~C05 {}",desc)).execute(&**self).await?;
        Ok(())
    }
    pub async fn send_distrib(
        &self,
        did: &str,
        pg: &GachaPg,
        data: &[ItemCode],
        cid: i32,
        pedia: &ItemPedia,
    ) -> Result<(), PgCustomError> {
        sqlx::query("UPDATE discord set gacha=$1,pity=$2 where discord_id=$3")
            .bind(pg.ticket)
            .bind(pg.pity)
            .bind(did)
            .execute(&**self)
            .await?;
        if data.len() == 1 {
            let text = match data.first().unwrap().text(pedia) {
                Some(x) => x,
                None => {
                    return Err(PgCustomError::from("no item in the data"));
                }
            };
            self.send_item(
                data,
                cid,
                &text,
                &format!("congratulation on getting {}", &text),
            )
            .await?;
            return Ok(());
        }
        self.send_item(
            data,
            cid,
            "Multi Gacha Rewards",
            "cant list all reward in description",
        )
        .await?;
        Ok(())
    }
    pub async fn bounty_transaction(&self, did: &str, price: i32) -> Result<(), PgCustomError> {
        sqlx::query("UPDATE discord set bounty=bounty-$1 where discord_id=$2")
            .bind(price)
            .bind(did)
            .execute(&**self)
            .await?;
        Ok(())
    }
    pub async fn bounty_event(&self, did: &str, event: &Event) -> Result<(), PgCustomError> {
        sqlx::query(
            "UPDATE discord set bounty=$1,gacha=$2,gold=$3,silver=$4,bronze=$5,latest_bounty=$6
            ,latest_bounty_time=$7,title=$8 where discord_id=$9",
        )
        .bind(event.bounty)
        .bind(event.gacha)
        .bind(event.gold)
        .bind(event.silver)
        .bind(event.bronze)
        .bind(&event.latest_bounty)
        .bind(event.latest_bounty_time)
        .bind(event.title)
        .bind(did)
        .execute(&**self)
        .await?;
        Ok(())
    }
    pub async fn bounty_all(&self, gift: i32) -> Result<(), PgCustomError> {
        sqlx::query("UPDATE discord set bounty=bounty+$1 where bounty is not null")
            .bind(gift)
            .execute(&**self)
            .await?;
        Ok(())
    }
    pub async fn jelewelry(&self, did: &str, bought: i32) -> Result<(), PgCustomError> {
        let user = self.get_user_data(did).await?;
        sqlx::query("UPDATE users set gacha_premium=coalesce(gacha_premium,0)+$1 where id=$2")
            .bind(bought)
            .bind(user.rid)
            .execute(&**self)
            .await?;
        Ok(())
    }
    pub async fn market(
        &self,
        did: &str,
        data: &ItemCode,
        cid: i32,
        price: Option<i32>,
        pedia: &ItemPedia,
    ) -> Result<(), PgCustomError> {
        if let Some(x) = price {
            self.bounty_transaction(did, x).await?;
        }
        let array = [data.clone()];
        let byte = Bitwise::new(&array);
        sqlx::query("INSERT into distribution (character_id,data,type,bot,event_name,description) Values ($1,$2,1,true,$3,$4)").bind(cid).bind(byte.multiple_item().unwrap()).bind(data.text(pedia).unwrap()).bind("~C05 The item distributed by admin").execute(&**self).await?;
        Ok(())
    }
    pub async fn market_user(
        &self,
        did: &str,
        data: &ItemCode,
        cid: i32,
        price: u32,
        pedia: &ItemPedia,
    ) -> Result<(), PgCustomError> {
        self.bounty_transaction(did, price as i32).await?;
        let array = [data.clone()];
        let byte = Bitwise::new(&array);
        sqlx::query("INSERT into distribution (character_id,data,type,bot,event_name,description) Values ($1,$2,1,true,$3,$4)").bind(cid).bind(byte.multiple_item().unwrap()).bind(data.text(pedia).unwrap()).bind("~C05 The market transaction delivery").execute(&**self).await?;
        Ok(())
    }
}
