use sqlx::{Pool, Postgres,Row, FromRow};
use bcrypt::{verify,hash};
use std::{time::SystemTime, fs::File};
use std::io::prelude::*;
use chrono::NaiveDateTime;
use crate::commands::binded::transfer::FileSave;

use super::{PgConn,BitwiseError};

#[derive(Debug,FromRow)]
pub struct AccountData {
    pub id: i32,
    pub username:String
}
#[derive(Debug,FromRow)]
pub struct SaveData {
    pub savedata: Option<Vec<u8>>,
    pub decomyset: Option<Vec<u8>>,
    pub hunternavi: Option<Vec<u8>>,
    pub otomoairou: Option<Vec<u8>>,
    pub partner: Option<Vec<u8>>,
    pub platedata: Option<Vec<u8>>,
    pub platebox: Option<Vec<u8>>,
    pub platemyset: Option<Vec<u8>>,
    pub rengokudata: Option<Vec<u8>>,
    pub savemercenary: Option<Vec<u8>>,
    pub skin_hist: Option<Vec<u8>>,
}
impl<'a> PgConn<'a>{
    pub async fn check_user_password(&self,cid:i32,pass:&str)->Result<bool,sqlx::Error>{
        check_password(&self.pool, cid, pass).await
    }
    pub async fn change_user_password(&self,pass:&str,cid:i32)->Result<(),BitwiseError>{
        let uid:i64 = sqlx::query("SELECT user_id FROM characters where id=$1").bind(cid)
            .fetch_one(&self.pool).await?.try_get("user_id")?;
        let hased = bcrypt::hash(pass, 10).unwrap_or_default();
        sqlx::query(&format!("UPDATE users SET password='{hased}' where id={uid}")).execute(&self.pool).await?;
        Ok(())
    }
    pub async fn create_account(&self,user:&str,pass:&str,reg:bool)->Result<AccountData,BitwiseError>{
        let uid;
        if reg{
            uid = create_account(&self.pool, user, pass).await?;
        }else {
            uid = sqlx::query_as::<_,AccountData>("SELECT id,username from users where username=$1").bind(user).fetch_one(&self.pool).await?;
        }
        use_history(&self.pool, &self.did, uid.id as i64).await?;
        Ok(uid)
    }
    pub async fn switch(&self,cid:i32)->Result<(),BitwiseError>{
        sqlx::query("INSERT INTO discord (discord_id,char_id,gacha) VALUES ($1,$2,100) ON CONFLICT (discord_id) DO UPDATE SET char_id=$2").bind(&self.did).bind(cid).execute(&self.pool).await?;
        Ok(())
    }
    pub async fn reset_cd(&self)->Result<(),sqlx::Error>{
        sqlx::query("UPDATE discord SET transfercd=0 WHERE discord_id=$1").bind(&self.did).execute(&self.pool).await?;
        Ok(())
    }
    pub async fn send_save(&self,cid:i32)->Result<SaveData,BitwiseError>{
        Ok(sqlx::query_as("SELECT * FROM characters WHERE id=$1").bind(cid).fetch_one(&self.pool).await?)
    }
    pub async fn transfer_cd(&self)->Result<(bool,i64),sqlx::Error>{
        let cd:i64 = sqlx::query("SELECT transfercd from discord where discord_id=$1").bind(&self.did).fetch_one(&self.pool).await?.try_get("transfercd")?;
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        if now > cd as u64{
            let week = now + 24*60*60;
            sqlx::query("UPDATE discord SET transfercd=$2 where discord_id=$1").bind(&self.did).bind(week as i64).execute(&self.pool).await?;
            return Ok((true,week as i64));
        }
        Ok((false,cd))
    }
    pub async fn transfer_file(&self,file:&FileSave,cid:i32)->Result<(),BitwiseError>{
        sqlx::query(&format!("UPDATE characters SET {}=$1 WHERE id=$2",&file.name)).bind(file.bin.as_slice()).bind(cid).execute(&self.pool).await?;
        Ok(())
    }
    pub async fn purge(&self)->Result<(),BitwiseError>{
        sqlx::query("DELETE from discord_register WHERE discord_id=$1").bind(&self.did).execute(&self.pool).await?;
        sqlx::query("DELETE from discord WHERE discord_id=$1").bind(&self.did).execute(&self.pool).await?;
        Ok(())
    }
}
impl SaveData {
    fn to_file(&self)->Result<(),std::io::Error>{
        if let Some(x) = &self.savedata{
            let mut y = File::create("./save/savefile.bin").unwrap();
            y.write_all(x.as_slice())?;
        }
        Ok(())
    }
}

async fn switch_character(cid:i32,did:&str,pool:&Pool<Postgres>)->Result<(),sqlx::Error>{
    sqlx::query("INSERT INTO discord (discord_id,char_id,gacha) VALUES ($1,$2,100) ON CONFLICT (discord_id) DO UPDATE SET char_id=$2").bind(did).bind(cid).execute(pool).await?;
    Ok(())
}
async fn get_save(pool:&Pool<Postgres>,cid:i32)->Result<SaveData,sqlx::Error>{
    sqlx::query_as("SELECT * FROM characters WHERE id=$1").bind(cid).fetch_one(pool).await
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
    // use super::*;
    // // use super::super::connection;
    // use crate::get_config;
    // use crate::PgConn;
    // use super::super::card::user_check;

    // #[tokio::test]
    // async fn test_user_creation(){
    //     let pool = connection(&get_config().unwrap()).await.unwrap();
    //     purge(&pool, "455622761168109569").await.unwrap();
    //     dell_acc(&pool, "grahamisdead").await.unwrap();
    //     let x = user_check("455622761168109569", &pool).await.unwrap();
    //     println!("{x:?}");
    //     pool.close().await;
    // }
    // #[tokio::test]
    // async fn test_save(){
    //     let pool = connection(&get_config().unwrap()).await.unwrap();
    //     let x = get_save(&pool,843).await.unwrap();
    //     assert_eq!((),x.to_file().unwrap());
    //     pool.close().await;
    // }
    // #[tokio::test]
    // async fn test_cd(){
    //     let init = get_config().unwrap();
    //     let did = init.discord.author_id.to_string();
    //     let mut pg = PgConn::create(&init, did).await.unwrap();
    //     let cd = pg.transfer_cd().await.unwrap();
    //     println!("{cd:?}");
    //     pg.close().await;
    // }
    // #[tokio::test]
    // async fn test_cd(){
    //     let init = get_config().unwrap();
    //     let did = init.discord.author_id.to_string();
    //     let mut pg = PgConn::create(&init, did).await.unwrap();
    //     let file = std::fs::read("./save/455622761168109569_savedata.bin").unwrap();
    //     let savefile = FileSave{bin:file,name:"savedata".to_string()};
    //     let data = pg.get_user_data().await.unwrap();
    //     pg.transfer_file(&savefile,data.cid).await.unwrap();
    //     pg.close().await;
    // }
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
