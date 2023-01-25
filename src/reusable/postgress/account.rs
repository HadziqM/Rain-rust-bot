use sqlx::{Pool, Postgres,Row};
use bcrypt::{verify,hash};
use std::time::SystemTime;
use chrono::NaiveDateTime;

use super::PgConn;

impl<'a> PgConn<'a>{
    pub async fn check_user_password(&self,cid:i32,pass:&str)->Result<bool,sqlx::Error>{
        check_password(&self.pool, cid, pass).await
    }
    pub async fn change_user_password(&self,pass:&str)->Result<(),sqlx::Error>{
        let cid = self.get_char_id().await?;
        change_password(&self.pool, cid, pass).await
    }
    pub async fn create_account(&self,user:&str,pass:&str)->Result<i64,sqlx::Error>{
        let uid = create_account(&self.pool, user, pass).await?;
        use_history(&self.pool, &self.did, uid).await?;
        Ok(uid)
    }
}



async fn check_password(pool:&Pool<Postgres>,cid:i32,pass:&str)->Result<bool,sqlx::Error>{
    let uid:i64 = sqlx::query(&format!("SELECT user_id FROM characters where id={cid}"))
        .fetch_one(pool).await?.try_get("user_id")?;
    let hash:String = sqlx::query(&format!("SELECT password FROM users where id={uid}"))
        .fetch_one(pool).await?.try_get("password")?;
    Ok(verify(pass,&hash).unwrap_or_default())
}
async fn change_password(pool:&Pool<Postgres>,cid:i32,pass:&str)->Result<(),sqlx::Error>{
    let uid:i64 = sqlx::query(&format!("SELECT user_id FROM characters where id={cid}"))
        .fetch_one(pool).await?.try_get("user_id")?;
    let hased = bcrypt::hash(pass, 10).unwrap_or_default();
    sqlx::query(&format!("UPDATE users SET password='{hased}' where id={uid}")).execute(pool).await?;
    Ok(())
}
fn get_naive()->Option<NaiveDateTime>{
    let amonth = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()+30*24*60*60;
    NaiveDateTime::from_timestamp_opt(amonth as i64,0)
}
pub async fn create_account(pool:&Pool<Postgres>,user:&str,pass:&str)->Result<i64,sqlx::Error>{
    let hash = hash(pass, 10).unwrap_or_default();
    sqlx::query(&format!("INSERT INTO users (username,password) VALUES ('{user}','{hash}')")).execute(pool).await?;
    Ok(sqlx::query(&format!("SELECT id FROM users WHERE username='{user}'")).fetch_one(pool).await?.try_get("id")?)
}

async fn use_history(pool:&Pool<Postgres>,did:&str,uid:i64)->Result<(),sqlx::Error>{
    sqlx::query(&format!("INSERT INTO discord_register (discord_id,user_id) VALUES ('{did}',{uid})")).execute(pool).await?;
    Ok(())
}
#[cfg(test)]
mod test_postgres{
    use super::*;
    use super::super::connection;
    use crate::get_config;

    #[tokio::test]
    async fn test_pass_validator() {
        let pool = connection(&get_config().unwrap()).await.unwrap();
        change_password(&pool, 843, "trustme").await.unwrap();
        let x = check_password(&pool,843,"trustme").await.unwrap();
        assert_eq!(x,true);
        pool.close().await;
    }
    #[test]
    fn test_naive() {
        println!("{:?}",get_naive().unwrap())
    }
    // #[tokio::test]
    // async fn test_user_creation(){
    //     let pool = connection(&get_config().unwrap()).await.unwrap();
    //     create_account(&pool, "dumbacc6978", "xxxyyyxxx").await.unwrap();
    //     create_account(&pool, "dumbacc6978", "asdasdsad").await.unwrap_err();
    //     pool.close().await;
    // }
    #[tokio::test]
    async fn make_hystory(){
        let pool = connection(&get_config().unwrap()).await.unwrap();
        use_history(&pool, "12321413121", 6000).await.unwrap();
        pool.close().await;
    }
}
