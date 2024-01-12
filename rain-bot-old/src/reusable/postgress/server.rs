use sqlx::FromRow;

use super::{BitwiseError, PgConn};

#[derive(Debug, FromRow)]
pub struct Servers {
    pub name: String,
    pub cp: i32,
    pub land: i32,
    pub description: String,
}
impl<'a> PgConn<'a> {
    pub async fn get_server(&self) -> Result<Vec<Servers>, BitwiseError> {
        Ok(sqlx::query_as::<_, Servers>(
            "Select current_players as cp,land,
world_name as name
,world_description as description from servers",
        )
        .fetch_all(&self.pool)
        .await?)
    }
    pub async fn psn(&self, psn: &str, id: i32) -> Result<(), BitwiseError> {
        sqlx::query("update users set psn_id=$1 where id=$2")
            .bind(psn)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
