use sqlx::Row;

use super::{BitwiseError,PgConn};

impl PgConn<'_>{
    async fn guild_id(&self,cid:i32)->Result<i32,BitwiseError>{
        let ids = sqlx::query("select guild_id from guild_characters where character_id=$1").bind(cid).fetch_one(&self.pool).await?;
        Ok(ids.try_get("guild_id")?)
    }
    pub async fn guild_food(&self,cid:i32,ids:i32,level:i32,exp:i32)->Result<(),BitwiseError>{
        let gid = self.guild_id(cid).await?;
        sqlx::query("insert into guild_meals (guild_id,meal_id,level,expires) value($1,$2,$3,$4)")
            .bind(gid).bind(ids).bind(level).bind(exp).execute(&self.pool).await?;
        Ok(())
    }
    pub async fn guild_rp(&self,cid:i32,rp:i32)->Result<(),BitwiseError>{
        let gid = self.guild_id(cid).await?;
        sqlx::query("update guilds set rank_rp=rank_rp+$1 where id=$2").bind(rp).bind(gid).execute(&self.pool).await?;
        Ok(())
    }
}
