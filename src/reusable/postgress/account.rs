use sqlx::{Pool, Postgres,Row, FromRow};
use bcrypt::{verify,hash};
use std::time::SystemTime;
use chrono::NaiveDateTime;

use super::PgConn;

#[derive(Debug,FromRow)]
pub struct AccountData {
    pub id: i32,
    pub username:String
}
impl<'a> PgConn<'a>{
    pub async fn check_user_password(&self,cid:i32,pass:&str)->Result<bool,sqlx::Error>{
        check_password(&self.pool, cid, pass).await
    }
    pub async fn change_user_password(&self,pass:&str)->Result<bool,sqlx::Error>{
        let check = self.get_user_data().await?;
        if check.cid != 0 || check.rid != 0{
            let cid = self.get_char_id().await?;
            change_password(&self.pool, cid.0, pass).await?;
            return Ok(true);
        }
        Ok(false)
    }
    pub async fn create_account(&self,user:&str,pass:&str)->Result<Option<AccountData>,sqlx::Error>{
        let check = self.get_user_data().await?;
        if check.cid != 0 || check.rid != 0{
            return Ok(None);
        }
        let uid = create_account(&self.pool, user, pass).await?;
        use_history(&self.pool, &self.did, uid.id as i64).await?;
        Ok(Some(uid))
    }
    pub async fn switch(&self,cid:i32)->Result<(),sqlx::Error>{
        switch_character(cid, &self.did, &self.pool).await
    }
}
async fn switch_character(cid:i32,did:&str,pool:&Pool<Postgres>)->Result<(),sqlx::Error>{
    sqlx::query("INSERT INTO discord (discord_id,char_id,gacha) VALUES ($1,$2,100) ON CONFLICT (discord_id) DO UPDATE SET char_id=$2").bind(did).bind(cid).execute(pool).await?;
    Ok(())
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
// pub async fn create_account(pool:&Pool<Postgres>,user:&str,pass:&str)->Result<i64,sqlx::Error>{
//     let hash = hash(pass, 10).unwrap_or_default();
//     sqlx::query(&format!("INSERT INTO users (username,password) VALUES ('{user}','{hash}')")).execute(pool).await?;
//     Ok(sqlx::query(&format!("SELECT id FROM users WHERE username='{user}'")).fetch_one(pool).await?.try_get("id")?)
// }
async fn create_account(pool:&Pool<Postgres>,user:&str,pass:&str)->Result<AccountData,sqlx::Error>{
    let hash = hash(pass, 10).unwrap_or_default();
    let time = get_naive().unwrap_or_default();
    let idk = sqlx::query_as::<_,AccountData>("INSERT INTO users (username,password,return_expires) VALUES ($1,$2,$3) RETURNING id,username").bind(user).bind(&hash).bind(time).fetch_one(pool).await?;
    Ok(idk)
}
async fn purge(pool:&Pool<Postgres>,did:&str)->Result<(),sqlx::Error>{
    sqlx::query("DELETE from discord_register WHERE discord_id=$1").bind(did).execute(pool).await?;
    sqlx::query("DELETE from discord WHERE discord_id=$1").bind(did).execute(pool).await?;
    Ok(())
}
async fn dell_acc(pool:&Pool<Postgres>,username:&str)->Result<(),sqlx::Error>{
    sqlx::query("DELETE from users WHERE username=$1").bind(username).execute(pool).await?;
    Ok(())
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
    use super::super::card::user_check;

    #[tokio::test]
    async fn test_user_creation(){
        let pool = connection(&get_config().unwrap()).await.unwrap();
        purge(&pool, "455622761168109569").await.unwrap();
        dell_acc(&pool, "grahamisdead").await.unwrap();
        let x = user_check("455622761168109569", &pool).await.unwrap();
        println!("{x:?}");
        pool.close().await;
    }
    // #[tokio::test]
    // async fn test_user_creation(){
    //     let pool = connection(&get_config().unwrap()).await.unwrap();
    //     let x = test_create_account(&pool, "ghrhamxx567", "asdasdsad").await.unwrap();
    //     println!("{x:?}");
    //     pool.close().await;
    // }
    // #[tokio::test]
    // async fn test_pass_validator() {
    //     let pool = connection(&get_config().unwrap()).await.unwrap();
    //     change_password(&pool, 843, "trustme").await.unwrap();
    //     let x = check_password(&pool,843,"trustme").await.unwrap();
    //     assert_eq!(x,true);
    //     pool.close().await;
    // }
    // #[test]
    // fn test_naive() {
    //     println!("{:?}",get_naive().unwrap())
    // }
    // #[tokio::test]
    // async fn test_user_creation(){
    //     let pool = connection(&get_config().unwrap()).await.unwrap();
    //     create_account(&pool, "dumbacc6978", "xxxyyyxxx").await.unwrap();
    //     create_account(&pool, "dumbacc6978", "asdasdsad").await.unwrap_err();
    //     pool.close().await;
    // }
    // #[tokio::test]
    // async fn make_hystory(){
    //     let pool = connection(&get_config().unwrap()).await.unwrap();
    //     use_history(&pool, "12321413121", 6000).await.unwrap();
    //     pool.close().await;
    // }
}
