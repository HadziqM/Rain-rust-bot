use sqlx::FromRow;

use super::{PgConn,BitwiseError};



#[derive(Debug,FromRow)]
pub struct Servers {
    pub name:String,
    pub cp:i32,
    pub land:i32,
    pub description:String,
}
impl<'a> PgConn<'a> {
    pub async fn get_server(&self)->Result<Vec<Servers>,BitwiseError>{
        Ok(sqlx::query_as::<_,Servers>("Select current_players as cp,land,
world_name as name
,world_description as description from servers").fetch_all(&self.pool).await?)
    }
}
