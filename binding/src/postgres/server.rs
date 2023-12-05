use sqlx::FromRow;
use super::{Db,PgCustomError};



#[derive(Debug,FromRow)]
pub struct Servers {
    pub name:String,
    pub cp:i32,
    pub land:i32,
    pub description:String,
}
impl Db {
    pub async fn get_server(&self)->Result<Vec<Servers>,PgCustomError>{
        Ok(sqlx::query_as::<_,Servers>("Select current_players as cp,land,
world_name as name
,world_description as description from servers").fetch_all(&**self).await?)
    }
}
