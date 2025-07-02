use chrono::{DateTime, Utc};
use common::database::{formatted::FormattedUserData, raw::DbCard};
use sqlx::{Row, prelude::FromRow};

use crate::*;

#[derive(Debug, FromRow, Clone)]
pub struct DbDiscordEvent {
    id: i32,
    discord_id: String,
    benefit: i32,
    bounty_coin: i32,
    gacha_ticket: i32,
    gacha_pity: i32,
    bounty_cd: DateTime<Utc>,
    transfer_cd: DateTime<Utc>,
}

impl Db {
    /// fetch basic event data
    pub async fn fetch_event(&self, discord_id: impl ToString) -> DbResult<DbDiscordEvent> {
        Ok(
            sqlx::query_as::<_, DbDiscordEvent>("SELECT * FROM event WHERE discord_id=$1")
                .bind(discord_id.to_string())
                .fetch_one(self.lite.pool())
                .await?,
        )
    }

    /// update event
    pub async fn update_event(&self, event: &DbDiscordEvent) -> DbResult<()> {
        sqlx::query(
            "UPDATE discord SET
            benefit = $1,
            bounty_coin = $2,
            gacha_ticket = $3,
            gacha_pity = $4,
            bounty_cd = $5,
            transfer_cd = $6,
        WHERE id = $7",
        )
        .bind(event.benefit)
        .bind(event.bounty_coin)
        .bind(event.gacha_ticket)
        .bind(event.gacha_pity)
        .bind(event.bounty_cd)
        .bind(event.transfer_cd)
        .execute(self.lite.pool())
        .await?;

        Ok(())
    }

    /// fetch all card
    pub async fn fetch_cards(&self, cids: &[i32]) -> DbResult<Vec<DbCard>> {
        if cids.is_empty() {
            return Ok(vec![]);
        }

        Ok(sqlx::query_as!(
            DbCard,
            r#"
        SELECT
            characters.id AS char_id,
            characters.user_id,
            characters.name,
            characters.gr,
            characters.hrp,
            characters.weapon_type,
            characters.last_login AS login,
            users.username,
            guilds.id AS guild_id,
            guilds.name AS guild_name
        FROM characters
        INNER JOIN users ON characters.user_id = users.id
        LEFT JOIN guild_characters ON characters.id = guild_characters.character_id
        LEFT JOIN guilds ON guild_characters.guild_id = guilds.id
        WHERE characters.id = ANY($1)
        "#,
            cids // &[i32]
        )
        .fetch_all(self.post.pool())
        .await?)
    }

    async fn fetch_all_character_id(&self, uid: i64) -> DbResult<Vec<i32>> {
        let rows = sqlx::query!("SELECT id FROM characters WHERE user_id = $1", uid)
            .fetch_all(self.post.pool())
            .await?;

        Ok(rows.into_iter().map(|r| r.id).collect())
    }

    /// fetch all card given uid
    pub async fn fetch_all_card(&self, user: i64) -> DbResult<Vec<DbCard>> {
        let cid = self.fetch_all_character_id(user).await?;
        Ok(self.fetch_cards(&cid).await?)
    }

    /// get account data
    pub async fn fetch_user_data(&self, discord_id: impl ToString) -> DbResult<FormattedUserData> {
        let did = discord_id.to_string();

        let x = sqlx::query(
            "SELECT user_id, char_id
            FROM discord
            WHERE discord_id=$1",
        )
        .bind(did)
        .fetch_one(self.post.pool())
        .await?;
        let uid = x.try_get::<i32, _>("user_id")?;
        let cid = x.try_get::<i32, _>("char_id")?;

        Ok(FormattedUserData { cid, uid })
    }
}
