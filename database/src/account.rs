use std::time::{SystemTime, UNIX_EPOCH};

use bcrypt::hash;
use common::database::raw::{DbAccountData, DbSaveData};
use sqlx::{query, query_as};

use super::*;

impl Db {
    pub async fn fetch_account(&self, uid: i32) -> DbResult<DbAccountData> {
        Ok(query_as!(
            DbAccountData,
            "SELECT id,username,password FROM users WHERE id = $1",
            uid
        )
        .fetch_one(&**self)
        .await?)
    }
    pub async fn check_password(&self, pass: &str, uid: i32) -> DbResult<bool> {
        let data = self.fetch_account(uid).await?;
        Ok(bcrypt::verify(pass, &data.password).unwrap())
    }
    pub async fn change_password(&self, pass: &str, uid: i32) -> DbResult<()> {
        let hased = bcrypt::hash(pass, 10).unwrap_or_default();
        sqlx::query!("UPDATE users SET password=$1 where id=$2", hased, uid)
            .execute(&**self)
            .await?;
        Ok(())
    }
    pub async fn change_psn(&self, psn: impl ToString, uid: i32) -> DbResult<()> {
        sqlx::query!(
            "UPDATE users SET psn_id=$1 where id=$2",
            psn.to_string(),
            uid
        )
        .execute(&**self)
        .await?;
        Ok(())
    }
    pub async fn add_account(
        &self,
        user: &str,
        pass: &str,
        discord_id: &str,
        bind: bool,
        psn: Option<String>,
    ) -> DbResult<DbAccountData> {
        let data;
        let id;
        if bind {
            data = query_as!(
                DbAccountData,
                "SELECT id,username,password FROM users WHERE username = $1",
                user
            )
            .fetch_one(&**self)
            .await?;
            if !bcrypt::verify(pass, &data.password).unwrap() {
                return Err("Password doesnt match the account".into());
            }
            id = query!("SELECT id FROM characters WHERE user_id=$1", data.id as i64)
                .fetch_one(&**self)
                .await?
                .id;
        } else {
            if query!("Select username from users where username=$1", user)
                .fetch_one(&**self)
                .await
                .is_ok()
            {
                return Err("Username already in use".into());
            }
            let hash = hash(pass, 10).unwrap_or_default();
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let month = now + 30 * 24 * 60 * 60;
            data = query_as!(DbAccountData,"INSERT INTO users (username,password,return_expires) VALUES ($1,$2,$3) RETURNING id,username,password",user,hash,month as i32)
                .fetch_one(&**self).await?;
            id = query!("INSERT INTO characters 
                            (user_id, is_female, is_new_character, name,unk_desc_string,
                            hrp, gr, weapon_type, last_login) VALUES($1, False, True, '', '', 0, 0, 0, $2) returning id",data.id as i64,now as i32).fetch_one(&**self).await?.id;
        }
        query!(
            "INSERT INTO discord_register (discord_id,user_id) VALUES ($1,$2)",
            discord_id,
            data.id
        )
        .execute(&**self)
        .await?;
        self.change_character(id, discord_id).await?;
        if let Some(psn) = psn {
            self.change_psn(&psn, data.id).await?;
        }
        Ok(data)
    }

    pub async fn change_character(&self, cid: i32, did: &str) -> DbResult<()> {
        query!("INSERT INTO discord (discord_id,char_id,gacha) VALUES ($1,$2,100) ON CONFLICT (discord_id) DO UPDATE SET char_id=$2",did,cid).execute(&**self).await?;
        Ok(())
    }

    pub async fn fetch_save(&self, cid: i32) -> DbResult<DbSaveData> {
        Ok(query_as!(
            DbSaveData,
            "SELECT savedata,decomyset,hunternavi,otomoairou,partner,platebox,platemyset,rengokudata,savemercenary,platedata FROM characters WHERE id=$1",
            cid
        )
        .fetch_one(&**self)
        .await?)
    }

    pub async fn transfer_file(&self, name: &str, file: Vec<u8>, cid: i32) -> DbResult<()> {
        sqlx::query(&format!("UPDATE characters SET {}=$1 WHERE id=$2", name))
            .bind(file.as_slice())
            .bind(cid)
            .execute(&**self)
            .await?;
        Ok(())
    }
    pub async fn purge(&self, did: &str) -> DbResult<()> {
        query!("DELETE from discord_register WHERE discord_id=$1", did)
            .execute(&**self)
            .await?;
        query!("DELETE from discord WHERE discord_id=$1", did)
            .execute(&**self)
            .await?;
        Ok(())
    }
}
