use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};

use crate::reusable::config::Init;


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


pub async fn connection(init:&Init) -> Result<Pool<Postgres>, sqlx::Error> {
    let url = format!("postgres://postgres:{}@{}:{}/{}",
        init.postgress.password,
        init.postgress.host,
        init.postgress.port,
        init.postgress.database);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url.as_str()).await?;
    Ok(pool)
}

pub async fn get_user(did:&str,conn:&Pool<Postgres>) -> Result<i32,sqlx::Error> {
    let row = sqlx::query(&format!("SELECT char_id FROM discord WHERE discord_id='{did}'"))
        .fetch_all(conn).await?;
    match row.first(){
        Some(d)=>Ok(d.get("char_id")),
        None=>Ok(0)
    }
}

pub async fn registered(did:&str,conn:&Pool<Postgres>)->Result<i32,sqlx::Error>{
    let row = sqlx::query(&format!("SELECT user_id FROM discord_register WHERE discord_id='{did}'"))
        .bind(did)
        .fetch_all(conn).await?;
    match row.first(){
        Some(d)=>Ok(d.get("user_id")),
        None=>Ok(0)
    }
}

pub async fn user_card(cid:i32,conn:&Pool<Postgres>)-> Result<Card,sqlx::Error>{
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

    #[tokio::test]
    async fn test_connection() {
        let mut res = "success".to_string();
        if let Err(why) = connection(&get_config().unwrap()).await{
            res = "fail".to_string();
            println!("{why}")
        };
        assert_eq!(&res,"success")
    }
    #[tokio::test]
    async fn test_user() {
        let pool = connection(&get_config().unwrap()).await.unwrap();
        let x = get_user("455622761168109569",&pool).await.unwrap();
        assert_eq!(x,843);
        let y = get_user("hahahahah",&pool).await.unwrap();
        assert_eq!(y,0);
    }
    #[tokio::test]
    async fn test_register() {
        let pool = connection(&get_config().unwrap()).await.unwrap();
        let x = registered("455622761168109569",&pool).await.unwrap();
        assert_eq!(x,2597);
    }
    #[tokio::test]
    async fn test_card() {
        let pool = connection(&get_config().unwrap()).await.unwrap();
        let x = user_card(843,&pool).await.unwrap();
        println!("{:?}",x);
        assert_eq!(1,1)
    }
}
