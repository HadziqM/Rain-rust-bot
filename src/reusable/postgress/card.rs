use sqlx::{Pool, Postgres, Row};
use super::PgConn;


#[derive(Debug)]
pub struct Card {
    pub char_id:i32,
    pub user_id:i64,
    pub username:String,
    pub name:String,
    pub login:i32,
    pub female:bool,
    pub hrp:i32,
    pub gr:i32,
    pub guild:String,
    pub guild_id:i64,
    pub guild_lead:i32,
    pub weapon_type:i32,
}

#[derive(Debug)]
pub struct UserData {
    pub cid: i32,
    pub rid:i32
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
    pub async fn many_card(&self)->Result<Option<Vec<Card>>,sqlx::Error>{
        let user = self.already_create().await?;
        if user != 0{
            let cid = get_user_all(user, &self.pool).await?;
            let mut card = Vec::new();
            for i in cid{
                card.push(user_card(i, &self.pool).await?);
            }
            return Ok(Some(card));
        }
        Ok(None)
    }
    pub async fn get_card(&self)->Result<Option<Card>,sqlx::Error>{
        let user = self.get_user_data().await?;
        if user.cid != 0{
            return Ok(Some(user_card(user.cid, &self.pool).await?));
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
            female: true,
            hrp: 0,
            gr: 0,
            guild: String::new(),
            guild_id: 0,
            guild_lead: 0,
            weapon_type: 0 }
    }
}

async fn user_check(did:&str,pool:&Pool<Postgres>)->Result<UserData,sqlx::Error>{
    let cid = get_user(did,pool).await?;
    if cid == 0 {
        return Ok(UserData { cid: 0, rid:registered(did, &pool).await? });
    }
    Ok(UserData { cid, rid: 0 })
}

async fn get_user(did:&str,conn:&Pool<Postgres>) -> Result<i32,sqlx::Error> {
    let row = sqlx::query(&format!("SELECT char_id FROM discord WHERE discord_id='{did}'"))
        .fetch_all(conn).await?;
    match row.first(){
        Some(d)=>Ok(d.get("char_id")),
        None=>Ok(0)
    }
}
async fn get_user_all(uid:i32,conn:&Pool<Postgres>) -> Result<Vec<i32>,sqlx::Error> {
    let row = sqlx::query(&format!("SELECT char_id FROM discord WHERE user_id={uid}"))
        .fetch_all(conn).await?;
    let mut cid:Vec<i32> = Vec::new();
    for i in row{
        cid.push(i.get("char_id"))
    }
    Ok(cid)
}
async fn get_user_name(did:&str,conn:&Pool<Postgres>) -> Result<(i32,String),sqlx::Error> {
    let row = sqlx::query(&format!("SELECT char_id username FROM discord WHERE discord_id='{did}'"))
        .fetch_all(conn).await?;
    match row.first(){
        Some(d)=>Ok((d.get("char_id"),d.get("username"))),
        None=>Ok((0,String::new()))
    }
}

async fn registered(did:&str,conn:&Pool<Postgres>)->Result<i32,sqlx::Error>{
    let row = sqlx::query(&format!("SELECT user_id FROM discord_register WHERE discord_id='{did}'"))
        .bind(did)
        .fetch_all(conn).await?;
    match row.first(){
        Some(d)=>Ok(d.get("user_id")),
        None=>Ok(0)
    }
}

async fn user_card(cid:i32,conn:&Pool<Postgres>)-> Result<Card,sqlx::Error>{
    let pg_guild = sqlx::query(&format!("SELECT guild_id FROM guild_characters WHERE character_id={cid}")).fetch_one(conn).await?;
    let pg_char = sqlx::query(&format!("SELECT user_id,is_female,name,gr,hrp,weapon_type,last_login FROM characters WHERE id={cid}")).fetch_one(conn).await?;
    let gid:i64 = pg_guild.try_get("guild_id")?;
    let pg_gc = sqlx::query(&format!("SELECT name,leader_id FROM guilds WHERE id={gid}")).fetch_one(conn).await?;
    let uid:i64 = pg_char.try_get("user_id")?;
    let user = sqlx::query(&format!("SELECT username FROM users WHERE id={uid}")).fetch_one(conn)
        .await?;
    Ok(Card { char_id: cid,
        user_id: uid,
        username: user.try_get("username")?,
        name: pg_char.try_get("name")?,
        login: pg_char.try_get("last_login")?,
        female: pg_char.try_get("is_female")?,
        hrp: pg_char.try_get("hrp")?,
        gr: pg_char.try_get("gr")?,
        guild: pg_gc.try_get("name")?,
        guild_id: gid,
        guild_lead: pg_gc.try_get("leader_id")?,
        weapon_type: pg_char.try_get("weapon_type")?
    })
}


#[cfg(test)]
mod postgres_test{
    use crate::reusable::config::get_config;

    use super::*;
    use super::super::connection;

    #[tokio::test]
    async fn test_user() {
        let pool = connection(&get_config().unwrap()).await.unwrap();
        let x = get_user("455622761168109569",&pool).await.unwrap();
        assert_eq!(x,843);
        let y = get_user("hahahahah",&pool).await.unwrap();
        assert_eq!(y,0);
        pool.close().await;
    }
    #[tokio::test]
    async fn test_register() {
        let pool = connection(&get_config().unwrap()).await.unwrap();
        let x = registered("455622761168109569",&pool).await.unwrap();
        assert_eq!(x,2597);
        let y = registered("asdaw7ey1wquqsada",&pool).await.unwrap();
        assert_eq!(y,0);
        pool.close().await;
    }
    #[tokio::test]
    async fn test_card() {
        let pool = connection(&get_config().unwrap()).await.unwrap();
        let x = user_card(843,&pool).await.unwrap();
        println!("{:?}",x);
        assert_eq!(1,1);
        pool.close().await;
    }
}
