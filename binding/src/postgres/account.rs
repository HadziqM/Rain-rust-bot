use std::time::SystemTime;

use crate::utils::MyTime;

use super::{Db, PgCustomError};
use bcrypt::{hash, verify};
use chrono::NaiveDateTime;
use sqlx::{FromRow, Row};

#[derive(Debug, FromRow)]
pub struct AccountData {
    pub id: i32,
    pub username: String,
    password: String,
}
#[derive(Debug, FromRow)]
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
}

pub struct FileSave {
    pub name: String,
    pub bin: Vec<u8>,
}

impl Db {
    pub async fn get_account_data(&self, uid: i32) -> Result<AccountData, PgCustomError> {
        Ok(sqlx::query_as::<_, AccountData>(
            "SELECT user.id as id, username,password from users where id = $1",
        )
        .bind(uid)
        .fetch_one(&**self)
        .await?)
    }
    pub async fn check_user_password(&self, uid: i32, pass: &str) -> Result<bool, PgCustomError> {
        let user = self.get_account_data(uid).await?;
        Ok(bcrypt::verify(pass, &user.password).unwrap())
    }
    pub async fn change_user_password(&self, pass: &str, uid: i32) -> Result<(), PgCustomError> {
        let hased = bcrypt::hash(pass, 10).unwrap_or_default();
        sqlx::query(&format!(
            "UPDATE users SET password='{hased}' where id={uid}"
        ))
        .execute(&**self)
        .await?;
        Ok(())
    }
    pub async fn create_account(
        &self,
        user: &str,
        pass: &str,
        psn: Option<String>,
        reg: bool,
        did: &str,
    ) -> Result<AccountData, PgCustomError> {
        let uid;
        if reg {
            if sqlx::query("Select username from users where username=$1")
                .bind(user)
                .fetch_one(&**self)
                .await
                .is_ok()
            {
                return Err(PgCustomError::Custom(
                    "The username already used by someone, please use another username".into(),
                ));
            }
            let hash = hash(pass, 10).unwrap_or_default();
            let time = get_naive().unwrap_or_default();
            uid = sqlx::query_as::<_,AccountData>("INSERT INTO users (username,password,return_expires) VALUES ($1,$2,$3) RETURNING id,username").bind(user).bind(&hash).bind(time).fetch_one(&**self).await?;
            sqlx::query("INSERT INTO characters 
                        (user_id, is_female, is_new_character, name,unk_desc_string,
                         hrp, gr, weapon_type, last_login) VALUES($1, False, True, '', '', 0, 0, 0, $2)").bind(uid.id)
                        .bind(MyTime::now() as i32).execute(&**self).await?;
        } else {
            let data = sqlx::query("SELECT id,username,password from users where username=$1")
                .bind(user)
                .fetch_one(&**self)
                .await?;
            if !verify(pass, &data.try_get::<String, _>("password")?).unwrap_or(true) {
                return Err(PgCustomError::Custom(
                    "Password doesnt match the account".into(),
                ));
            }
            uid = AccountData {
                username: data.try_get("username")?,
                password: data.get("password"),
                id: data.try_get("id")?,
            };
        }
        sqlx::query(&format!(
            "INSERT INTO discord_register (discord_id,user_id) VALUES ($1,$2)"
        ))
        .bind(did)
        .bind(uid.id)
        .execute(&**self)
        .await?;
        if let Some(psn) = psn {
            self.add_psn(&psn, uid.id).await?;
        }
        Ok(uid)
    }
    pub async fn add_psn(&self, psn: &str, id: i32) -> Result<(), PgCustomError> {
        match sqlx::query("select username from users where psn_id = ").fetch_one(&**self).await {
            Ok(x) =>
                Err(PgCustomError::Custom(format!("username {} have same psn id as you, duplicate psn will cause trouble, ask admin if you want to force psn id into this account and delete the others",x.get::<String,_>("username")))),
            Err(_) =>{
                sqlx::query("update users set psn_id=$1 where id=$2").bind(psn).bind(id).execute(&**self).await?;
                Ok(())
            }
        }
    }
    pub async fn force_psn(&self, psn: &str, uid: i32) -> Result<(), PgCustomError> {
        sqlx::query("update users set psn_id=NULL where psn_id=$1")
            .bind(psn)
            .execute(&**self)
            .await?;
        sqlx::query("update users set psn_id=$1 where id=$2")
            .bind(psn)
            .bind(uid)
            .execute(&**self)
            .await?;
        Ok(())
    }
    pub async fn switch(&self, cid: i32, did: &str) -> Result<(), PgCustomError> {
        sqlx::query("INSERT INTO discord (discord_id,char_id,gacha) VALUES ($1,$2,100) ON CONFLICT (discord_id) DO UPDATE SET char_id=$2").bind(did).bind(cid).execute(&**self).await?;
        Ok(())
    }
    pub async fn send_save(&self, cid: i32) -> Result<SaveData, PgCustomError> {
        Ok(sqlx::query_as("SELECT * FROM characters WHERE id=$1")
            .bind(cid)
            .fetch_one(&**self)
            .await?)
    }
    pub async fn transfer_file(&self, file: &FileSave, cid: i32) -> Result<(), PgCustomError> {
        sqlx::query(&format!(
            "UPDATE characters SET {}=$1 WHERE id=$2",
            &file.name
        ))
        .bind(file.bin.as_slice())
        .bind(cid)
        .execute(&**self)
        .await?;
        Ok(())
    }
    pub async fn purge(&self, did: &str) -> Result<(), PgCustomError> {
        sqlx::query("DELETE from discord_register WHERE discord_id=$1")
            .bind(did)
            .execute(&**self)
            .await?;
        sqlx::query("DELETE from discord WHERE discord_id=$1")
            .bind(did)
            .execute(&**self)
            .await?;
        Ok(())
    }
}

fn get_naive() -> Option<NaiveDateTime> {
    let amonth = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 30 * 24 * 60 * 60;
    NaiveDateTime::from_timestamp_opt(amonth as i64, 0)
}
