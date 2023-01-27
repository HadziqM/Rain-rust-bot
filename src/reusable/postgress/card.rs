use sqlx::{Pool, Postgres, Row, FromRow};
use super::PgConn;

#[derive(Debug,FromRow)]
pub struct Card {
    pub char_id: i32,
    pub user_id: i64,
    pub name:String,
    pub gr:i32,
    pub hrp:i32,
    pub login:i32,
    pub weapon_type:i32,
    pub username:String,
    pub guild_id:Option<i64>,
    pub guild_name:Option<String>
}

impl<'a> PgConn<'a>{
    pub async fn get_char_id(&self)-> Result<(i32,String),sqlx::Error>{
        get_user_name(&self.did, &self.pool).await
    }
    pub async fn already_create(&self)->Result<i32,sqlx::Error>{
        registered(&self.did, &self.pool).await
    }
    pub async fn get_user_data(&self)->Result<UserData,sqlx::Error>{
        user_check(&self.did, &self.pool).await
    }
    pub async fn many_card(&self,user:i32)->Result<Vec<Card>,sqlx::Error>{
        let cid = get_user_all(user, &self.pool).await?;
        let mut card = Vec::new();
        for i in cid{
            card.push(test_card(i, &self.pool).await?);
        }
        return Ok(card);
    }
    pub async fn get_card(&self)->Result<Option<Card>,sqlx::Error>{
        let user = self.get_user_data().await?;
        if user.cid != 0{
            return Ok(Some(test_card(user.cid, &self.pool).await?));
        };
        Ok(None)
    }
}

impl Card{
    pub fn default()->Card{
        Card { char_id: 0,
            user_id: 0,
            username: "deleted".to_string(),
            name: "deleted".to_string(),
            login: 0,
            hrp: 0,
            gr: 0,
            guild_name: None,
            guild_id: None,
            weapon_type: 0
        }
    }
}
#[derive(Debug,FromRow)]
pub struct UserData {
    pub cid:i32,
    pub rid:i32
}

pub async fn user_check(did:&str,pool:&Pool<Postgres>)->Result<UserData,sqlx::Error>{
    let cid = get_user(did,pool).await?;
    if cid == 0 {
        return Ok(UserData { cid: 0, rid:registered(did, &pool).await? });
    }
    Ok(UserData { cid, rid: 0 })
}

async fn get_user(did:&str,conn:&Pool<Postgres>) -> Result<i32,sqlx::Error> {
    let row = sqlx::query("SELECT char_id FROM discord WHERE discord_id=$1").bind(did)
        .fetch_all(conn).await?;
    match row.first(){
        Some(d)=>Ok(d.get("char_id")),
        None=>Ok(0)
    }
}
async fn get_user_all(uid:i32,conn:&Pool<Postgres>) -> Result<Vec<i32>,sqlx::Error> {
    let row = sqlx::query("SELECT id FROM characters WHERE user_id=$1").bind(uid)
        .fetch_all(conn).await?;
    let mut cid:Vec<i32> = Vec::new();
    for i in row{
        cid.push(i.get("id"))
    }
    Ok(cid)
}
async fn get_user_name(did:&str,conn:&Pool<Postgres>) -> Result<(i32,String),sqlx::Error> {
    let row = sqlx::query("SELECT user_id,username FROM discord_register LEFT OUTER JOIN users ON user_id=users.id WHERE discord_id=$1").bind(did)
        .fetch_all(conn).await?;
    match row.first(){
        Some(d)=>Ok((d.get("user_id"),d.get("username"))),
        None=>Ok((0,String::new()))
    }
}

async fn registered(did:&str,conn:&Pool<Postgres>)->Result<i32,sqlx::Error>{
    let row = sqlx::query("SELECT user_id FROM discord_register WHERE discord_id=$1")
        .bind(did)
        .fetch_all(conn).await?;
    match row.first(){
        Some(d)=>Ok(d.get("user_id")),
        None=>Ok(0)
    }
}

async fn test_card(cid:i32,conn:&Pool<Postgres>)->Result<Card,sqlx::Error>{
    let idk = sqlx::query_as::<_,Card>(
        "SELECT characters.id as char_id, user_id,characters.name as name,gr,hrp,weapon_type,
characters.last_login as login,username,guild_id,guilds.name as guild_name 
FROM characters 
INNER JOIN users ON characters.user_id = users.id 
LEFT OUTER JOIN guild_characters ON characters.id = guild_characters.character_id
LEFT OUTER JOIN guilds ON guild_characters.guild_id = guilds.id
WHERE characters.id=$1").bind(cid).fetch_one(conn).await?;
    Ok(idk)
}

// #[cfg(test)]
// mod postgres_test{
//     use crate::reusable::config::get_config;
//
//     use super::*;
//     use super::super::connection;

    // #[tokio::test]
    // async fn test_user() {
    //     let pool = connection(&get_config().unwrap()).await.unwrap();
    //     let x = test_card(843, &pool).await.unwrap();
    //     println!("{x:?}");
    //     pool.close().await;
    // }
    // #[tokio::test]
    // async fn test_user() {
    //     let pool = connection(&get_config().unwrap()).await.unwrap();
    //     let x = get_user("455622761168109569",&pool).await.unwrap();
    //     assert_eq!(x,843);
    //     let y = get_user("hahahahah",&pool).await.unwrap();
    //     assert_eq!(y,0);
    //     pool.close().await;
    // }
    // #[tokio::test]
    // async fn test_register() {
    //     let pool = connection(&get_config().unwrap()).await.unwrap();
    //     let x = registered("455622761168109569",&pool).await.unwrap();
    //     assert_eq!(x,2597);
    //     let y = registered("asdaw7ey1wquqsada",&pool).await.unwrap();
    //     assert_eq!(y,0);
    //     pool.close().await;
    // }
    // #[tokio::test]
    // async fn test_card() {
    //     let pool = connection(&get_config().unwrap()).await.unwrap();
    //     let x = user_card(843,&pool).await.unwrap();
    //     println!("{:?}",x);
    //     assert_eq!(1,1);
    //     pool.close().await;
    // }
// }
