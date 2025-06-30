use common::database::raw::{DbCard, DbEvent, DbUserData};

use crate::*;

impl Db {
    pub async fn fetch_event(&self, discord_id: impl ToString) -> DbResult<DbEvent> {
        Ok(sqlx::query_as!(
            DbEvent,
            "SELECT
                characters.name as name,char_id,bounty,gacha,pity,latest_bounty,latest_bounty_time,
                title,bronze,silver,gold
                FROM discord
                JOIN characters on discord.char_id=characters.id
                WHERE discord_id=$1",
            discord_id.to_string()
        )
        .fetch_one(self.pool())
        .await?)
    }

    pub async fn update_event(&self, event: &DbEvent) -> DbResult<()> {
        sqlx::query!(
            "UPDATE discord SET
            bounty = $1,
            gacha = $2,
            pity = $3,
            latest_bounty = $4,
            latest_bounty_time = $5,
            title = $6,
            bronze = $7,
            silver = $8,
            gold = $9
        WHERE char_id = $10",
            event.bounty,
            event.gacha,
            event.pity,
            event.latest_bounty,
            event.latest_bounty_time,
            event.title,
            event.bronze,
            event.silver,
            event.gold,
            event.char_id
        )
        .execute(self.pool())
        .await?;

        Ok(())
    }

    pub async fn fetch_card(&self, cid: i32) -> DbResult<DbCard> {
        Ok(sqlx::query_as!(
            DbCard,
            "SELECT characters.id as char_id, user_id,characters.name as name,gr,hrp,weapon_type,
            characters.last_login as login,username,guild_id,guilds.name as guild_name
            FROM characters
            INNER JOIN users ON characters.user_id = users.id
            LEFT OUTER JOIN guild_characters ON characters.id = guild_characters.character_id
            LEFT OUTER JOIN guilds ON guild_characters.guild_id = guilds.id
            WHERE characters.id=$1",
            cid
        )
        .fetch_one(self.pool())
        .await?)
    }

    async fn fetch_all_character_id(&self, uid: i64) -> DbResult<Vec<i32>> {
        let row = sqlx::query!("SELECT id FROM characters WHERE user_id=$1", uid)
            .fetch_all(self.pool())
            .await?;
        let mut cid = Vec::new();
        for i in row {
            cid.push(i.id)
        }
        Ok(cid)
    }

    pub async fn fetch_all_card(&self, user: i64) -> DbResult<Vec<DbCard>> {
        let cid = self.fetch_all_character_id(user).await?;
        let mut card = Vec::new();
        for i in cid {
            card.push(self.fetch_card(i).await?);
        }
        Ok(card)
    }

    pub async fn fetch_user_data(&self, discord_id: impl ToString) -> DbResult<DbUserData> {
        let did = discord_id.to_string();

        Ok(sqlx::query_as!(
            DbUserData,
            "SELECT user_id as uid, char_id as cid
            FROM discord_register
            LEFT OUTER JOIN discord ON discord_register.discord_id=discord.discord_id
            WHERE discord.discord_id=$1",
            did
        )
        .fetch_one(self.pool())
        .await?)
    }
}
