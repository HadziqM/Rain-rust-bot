use std::{collections::HashMap, io::LineWriter, time::{SystemTime, UNIX_EPOCH}};

use bcrypt::hash;
use common::database::{formatted::FormattedUserData, raw::{DbAccountData, DbSaveData}};
use sqlx::{query, query_as};

use super::*;

impl Db {
    /// check game account for binding
    pub async fn check_account(&self, username:&str , pass: &str) -> DbResult<DbAccountData> {
        let u =  query_as!(
            DbAccountData,
            "SELECT id,username,password FROM users WHERE username = $1",username)
            .fetch_one(self.post.pool())
            .await
            .map_err(|e|DbError::Custom(format!("Predicted Err: InGame Username doesnt exist, you can create new one\nOriginal Err: {e:?}")))?;
        if !bcrypt::verify(pass, &u.password).map_err(|e|DbError::Custom(format!("Bcrypt error at decrypt password, report this\nOriginal Err: {e:?}")))? {
            return Err(DbError::Custom("Password didnt match".into()));
        }

        Ok(u)
    }

    /// create new in game account
    pub async fn create_new_account(&self, user:&str, pass: &str, did: &str , psn: Option<String>) -> DbResult<FormattedUserData> {
        let mut post = self.post.pool().begin().await?;

        
        let hash = hash(pass, 10).unwrap_or_default();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let month = now + 30 * 24 * 60 * 60;
        let data = query_as!(DbAccountData,"INSERT INTO users (username,password,return_expires,psn_id) VALUES ($1,$2,$3,$4) RETURNING id,username,password",user,hash,month as i32,psn)
            .fetch_one(&mut *post).await?;
        let cid = query!("INSERT INTO characters
            (user_id, is_female, is_new_character, name,unk_desc_string,
            hrp, gr, weapon_type, last_login) VALUES($1, False, True, '', '', 0, 0, 0, $2) returning id",data.id as i64,now as i32).fetch_one(&mut *post).await?.id;

        self.bind_account(cid, data.id, did).await?;

        post.commit().await?;

        Ok(FormattedUserData {cid,uid:data.id})
    }

    /// change password
    pub async fn change_password(&self, pass: &str, uid: i32) -> DbResult<()> {
        let hased = bcrypt::hash(pass, 10).unwrap_or_default();
        sqlx::query!("UPDATE users SET password=$1 where id=$2", hased, uid)
            .execute(self.post.pool())
            .await?;
        Ok(())
    }

    
    /// update chatacter of existing account
    pub async fn change_character(&self, cid: i32, did: &str) -> DbResult<()> {
        query("UPDATE discord SET char_id=$2 WHERE discord_id=$1")
            .bind(did)
            .bind(cid)
            .execute(self.lite.pool())
            .await?;
        Ok(())
    }

    /// bind game account to discord
    pub async fn bind_account(&self, cid: i32, uid: i32, did: &str) -> DbResult<()> {
        let mut trans = self.lite.pool().begin().await?;

        query("INSERT INTO discord (discord_id,user_id,char_id) VALUES ($1.$2,$3)")
            .bind(did)
            .bind(uid)
            .bind(cid)
            .execute(&mut *trans)
            .await
            .map_err(|e|DbError::Custom(format!("Predicted Err: You already Has Discord Binded Purge the account to delete it\nOriginal Err : {e:?}")))?;

        query("INSERT INTO event (discord_id) VALUES ($1)")
            .bind(did)
            .execute(&mut *trans)
            .await?;

        trans.commit().await?;

        Ok(())
    }

    /// change in game account psn (or null to remove)
    pub async fn change_psn(&self, psn: Option<String>, uid: i32) -> DbResult<()> {
        sqlx::query!("UPDATE users SET psn_id=$1 where id=$2", psn, uid)
            .execute(self.post.pool())
            .await?;
        Ok(())
    }

    /// fetch save file of the selected character  
    pub async fn fetch_save(&self, cid: i32) -> DbResult<DbSaveData> {
        Ok(query_as!(
            DbSaveData,
            "SELECT savedata,decomyset,hunternavi,otomoairou,partner,platebox,platemyset,rengokudata,savemercenary,platedata FROM characters WHERE id=$1",
            cid
        )
        .fetch_one(self.post.pool())
        .await?)
    }

    /// transfer file
    pub async fn transfer_file(&self,msg_url:&str,files:HashMap<String,Vec<u8>> , cid: i32, did: &str) -> DbResult<()> {
        for (name,file) in files.into_iter() {
            
            query(&format!("UPDATE characters SET {name}=$1 WHERE id=$2"))
                .bind(file.as_slice())
                .bind(cid)
                .execute(self.post.pool())
                .await?;
        }

        query("INSERT INTO transfer_history (discord_id,message_url) VALUES ($1,$2)")
            .bind(did)
            .bind(msg_url)
            .execute(self.lite.pool())
            .await?;
        
        Ok(())
    }

    /// purge binding
    pub async fn purge(&self, did: &str) -> DbResult<()> {
        query("DELETE from discord WHERE discord_id=$1").bind(did)
            .execute(self.lite.pool())
            .await?;
        Ok(())
    }
}
