use crate::{Db, DbResult};
use common::item_code::ItemCode;

impl Db {
    //TODO: proper error handling
    /// sending distribution data
    pub async fn send_distribution(
        &self,
        ids: &[i32],
        code: &[ItemCode],
        name: impl ToString,
        desc: impl ToString,
    ) -> DbResult<()> {
        let len = ids.len();
        let names = vec![name.to_string(); len];
        let description = vec![desc.to_string(); len];
        let bot = vec![true; len];
        let distributor = vec![1; len];

        let mut keys = vec![];
        let mut count = vec![];
        let mut types = vec![];
        for i in code {
            keys.push(i.transform_key().unwrap());
            count.push(i.count as i32);
            types.push(i.types as i32);
        }

        sqlx::query!(
            "WITH di AS(
                INSERT INTO distribution
                    (character_id,type,bot,event_name,description)
                SELECT * FROM
                    UNNEST($1::INT[], $2::INT[], $3::bool[], $4::text[], $5::text[])
                RETURNING id
            )
            INSERT INTO distribution_items
                (distribution_id,item_type, item_id, quantity)
            SELECT di.id,u.type,u.key,u.count
            FROM di
                CROSS JOIN unnest($6::INT[],$7::INT[],$8::INT[])
                AS u(type,key,count)
            ",
            ids,
            &distributor,
            &bot,
            &names,
            &description,
            &types,
            &keys,
            &count
        )
        .execute(self.pool())
        .await?;

        Ok(())
    }

    pub async fn send_event(&self, ids: &[i32], bounty_coin: i32, gacha: i32) -> DbResult<()> {
        sqlx::query!(
            "
            UPDATE discord
            SET
                bounty = bounty + $1,
                gacha = gacha + $2
            WHERE char_id = ANY($3)
        ",
            bounty_coin,
            gacha,
            ids
        )
        .execute(self.pool())
        .await?;

        Ok(())
    }
}
