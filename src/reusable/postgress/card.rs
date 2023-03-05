use sqlx::{Row, FromRow};
use super::{PgConn,BitwiseError};

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
impl Default for Card{
    fn default()-> Self {
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

#[derive(FromRow,Clone,Debug)]
pub struct Event {
    pub bounty:i32,
    pub gacha:i32,
    pub pity:i32,
    pub latest_bounty:String,
    pub latest_bounty_time:i64,
    pub title:i32,
    pub bronze:i32,
    pub silver:i32,
    pub gold:i32,
    pub name:String,
    pub char_id:i32
}
#[derive(Debug,FromRow)]
pub struct UserData {
    pub cid:i32,
    pub rid:i32
}
impl<'a> PgConn<'a>{
    pub async fn get_event(&self)->Result<Event,BitwiseError>{
        Ok(sqlx::query_as::<_,Event>("Select characters.name as name,char_id,bounty,gacha,pity,latest_bounty,latest_bounty_time,title,bronze,silver,gold from discord inner join characters on discord.char_id=characters.id  where discord_id=$1").bind(&self.did).fetch_one(&self.pool).await?)
    }
    pub async fn get_user(&self)-> Result<(i32,String),BitwiseError>{
        let row = sqlx::query("SELECT user_id,username FROM discord_register LEFT OUTER JOIN users ON user_id=users.id WHERE discord_id=$1").bind(&self.did)
            .fetch_all(&self.pool).await?;
        match row.first(){
            Some(d)=>Ok((d.get("user_id"),d.get("username"))),
            None=>Ok((0,String::new()))
        }
    }
    pub(super) async fn get_char_id(&self)-> Result<i32,BitwiseError>{
        let row = sqlx::query("SELECT char_id FROM discord WHERE discord_id=$1").bind(&self.did)
            .fetch_all(&self.pool).await?;
        match row.first(){
            Some(d)=>Ok(d.get("char_id")),
            None=>Ok(0)
        }
    }
    async fn already_create(&self)->Result<i32,BitwiseError>{
        let row = sqlx::query("SELECT user_id FROM discord_register WHERE discord_id=$1")
            .bind(&self.did)
            .fetch_all(&self.pool).await?;
        match row.first(){
            Some(d)=>Ok(d.get("user_id")),
            None=>Ok(0)
        }
    }
    async fn get_all_cid(&self,uid:i32)->Result<Vec<i32>,BitwiseError>{
        let row = sqlx::query("SELECT id FROM characters WHERE user_id=$1").bind(uid)
            .fetch_all(&self.pool).await?;
        let mut cid:Vec<i32> = Vec::new();
        for i in row{
            cid.push(i.get("id"))
        }
        Ok(cid)
    }
    pub async fn get_user_data(&self)->Result<UserData,BitwiseError>{
        let cid = self.get_char_id().await?;
        if cid == 0 {
            return Ok(UserData { cid: 0, rid:self.already_create().await? });
        }
        Ok(UserData { cid, rid: 0 })
    }
    pub async fn get_user_data_long(&self)->Result<UserData,BitwiseError>{
        let cid = self.get_char_id().await?;
        let rid = self.already_create().await?;
        if cid == 0 {
            return Ok(UserData { cid: 0, rid });
        }
        if cid != 0 && rid == 0{
            let uid:i64 = sqlx::query("SELECT user_id FROM characters WHERE id=$1").bind(cid).fetch_one(&self.pool).await?.try_get("user_id")?;
            sqlx::query("INSERT INTO discord_register (discord_id,user_id) VALUES ($1,$2)").bind(&self.did)
            .bind(uid as i32).execute(&self.pool).await?;
            return Ok(UserData{cid,rid:uid as i32});
        }
        Ok(UserData{cid,rid})
    }
    pub async fn many_card(&self,user:i32)->Result<Vec<Card>,BitwiseError>{
        let cid = self.get_all_cid(user).await?;
        let mut card = Vec::new();
        for i in cid{
            card.push(self.get_card(i).await?);
        }
        return Ok(card);
    }
    pub async fn get_card(&self,cid:i32)->Result<Card,BitwiseError>{
        let idk = sqlx::query_as::<_,Card>(
            "SELECT characters.id as char_id, user_id,characters.name as name,gr,hrp,weapon_type,
            characters.last_login as login,username,guild_id,guilds.name as guild_name 
            FROM characters 
            INNER JOIN users ON characters.user_id = users.id 
            LEFT OUTER JOIN guild_characters ON characters.id = guild_characters.character_id
            LEFT OUTER JOIN guilds ON guild_characters.guild_id = guilds.id
            WHERE characters.id=$1").bind(cid).fetch_one(&self.pool).await?;
        Ok(idk)
    }
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
