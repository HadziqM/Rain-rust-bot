use super::{Db, PgCustomError};
use sqlx::Row;

#[derive(sqlx::FromRow, Debug)]
pub struct GuildList {
    pub id: i32,
    pub name: String,
}

#[derive(sqlx::FromRow)]
pub struct Guild {
    pub id: i32,
    pub name: String,
    pub rank_rp: i32,
    pub leader_id: i32,
    pub created: chrono::DateTime<chrono::Utc>,
    pub lead_name: String,
    pub discord_id: Option<String>,
}

impl Db {
    async fn guild_id(&self, cid: i32) -> Result<i64, PgCustomError> {
        if let Ok(x) = sqlx::query("select guild_id from guild_characters where character_id=$1")
            .bind(cid)
            .fetch_one(&**self)
            .await
        {
            return Ok(x.try_get("guild_id")?);
        }
        Ok(0)
    }
    async fn guild_count(&self, ids: i64) -> Result<i64, PgCustomError> {
        Ok(
            sqlx::query("select count(*) from guild_characters where guild_id=$1")
                .bind(ids)
                .fetch_one(&**self)
                .await?
                .try_get("count")?,
        )
    }
    pub async fn guild_food(
        &self,
        cid: i32,
        ids: i32,
        level: i32,
        exp: i32,
    ) -> Result<bool, PgCustomError> {
        let x = chrono::NaiveDateTime::from_timestamp_millis(exp as i64 * 1000).unwrap();
        let z: chrono::DateTime<chrono::Utc> =
            chrono::DateTime::from_naive_utc_and_offset(x, chrono::Utc);
        let gid = self.guild_id(cid).await?;
        if gid != 0 {
            sqlx::query(
                "insert into guild_meals (guild_id,meal_id,level,created_at) values ($1,$2,$3,$4)",
            )
            .bind(gid as i32)
            .bind(ids)
            .bind(level)
            .bind(z)
            .execute(&**self)
            .await?;
            return Ok(true);
        }
        Ok(false)
    }
    pub async fn guild_rp(&self, cid: i32, rp: i32) -> Result<bool, PgCustomError> {
        let gid = self.guild_id(cid).await?;
        if gid == 0 {
            return Ok(false);
        }
        sqlx::query("update guilds set rank_rp=rank_rp+$1 where id=$2")
            .bind(rp)
            .bind(gid as i32)
            .execute(&**self)
            .await?;
        Ok(true)
    }
    pub async fn guild_list(&self) -> Result<Vec<GuildList>, PgCustomError> {
        Ok(sqlx::query_as::<_, GuildList>("select id,name from guilds")
            .fetch_all(&**self)
            .await?)
    }
    pub async fn guild_search(&self, ids: i64) -> Result<(Guild, i64), PgCustomError> {
        let guild = sqlx::query_as(
            "select guilds.name as name,characters.name as lead_name,guilds.id as id,rank_rp,guilds.created_at as created,leader_id,discord_id 
            from guilds inner join characters on characters.id = guilds.leader_id 
            left outer join discord on discord.char_id = guilds.leader_id 
            where guilds.id=$1").bind(ids).fetch_one(&**self).await?;
        Ok((guild, self.guild_count(ids).await?))
    }
    pub async fn guild_join(&self, ids: i64, cid: i32) -> Result<(), PgCustomError> {
        if self.guild_id(cid).await? != 0 {
            return Err(PgCustomError::Custom(
                "you already have guild, leave your current guild to use this command".to_owned(),
            ));
        }
        if self.guild_count(ids).await? > 59 {
            return Err(PgCustomError::Custom(
                "the guild you selected is already full".to_owned(),
            ));
        }
        if let Err(why) = sqlx::query(
            "insert into guild_characters (guild_id,character_id,order_index) 
            values ($1,$2,(select max(order_index)+1 from guild_characters where guild_id = $1))",
        )
        .bind(ids)
        .bind(cid)
        .execute(&**self)
        .await
        {
            return Err(PgCustomError::from(why).into());
        }
        Ok(())
    }
}
