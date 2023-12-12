use sqlx::{Row, FromRow,Result};
use super::{Db,PgCustomError};
use serde::{Serialize,Deserialize};

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

#[derive(FromRow,Clone,Debug,Serialize,Deserialize)]
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

impl Db{
    pub async fn get_event(&self,did:&str)->Result<Event,PgCustomError>{
        Ok(sqlx::query_as::<_,Event>("Select characters.name as name,char_id,bounty,gacha,pity,latest_bounty,latest_bounty_time,title,bronze,silver,gold from discord inner join characters on discord.char_id=characters.id  where discord_id=$1").bind(did).fetch_one(&**self).await?)
    }
    pub async fn get_user(&self,did:&str)-> Result<(i32,String),PgCustomError>{
        let row = sqlx::query("SELECT user_id,username FROM discord_register LEFT OUTER JOIN users ON user_id=users.id WHERE discord_id=$1").bind(did)
            .fetch_all(&**self).await?;
        match row.first(){
            Some(d)=>Ok((d.get("user_id"),d.get("username"))),
            None=>Err(PgCustomError::from("user isn't registered yet try use register methode in guide"))
        }
    }
    pub async fn get_char_id(&self,did:&str)-> Result<i32,PgCustomError>{
        let row = sqlx::query("SELECT char_id FROM discord WHERE discord_id=$1").bind(did)
            .fetch_all(&**self).await?;
        match row.first(){
            Some(d)=>Ok(d.get("char_id")),
            None=>Err(PgCustomError::Custom(format!("<@{did}> isnt selected their chard yet tell them to use any command and press use")))
        }
    }
    async fn get_all_cid(&self,uid:i32)->Result<Vec<i32>,PgCustomError>{
        let row = sqlx::query("SELECT id FROM characters WHERE user_id=$1").bind(uid)
            .fetch_all(&**self).await
            .ok().ok_or(PgCustomError::from("user doesnt have any character, try to make one from game launcher and proceed till enter mezeporta"))?;
        let mut cid:Vec<i32> = Vec::new();
        for i in row{
            cid.push(i.get("id"))
        }
        Ok(cid)
    }
    pub async fn get_user_data(&self,did:&str)->Result<UserData,PgCustomError>{
        let rid = self.get_user(did).await?.0;
        let cid = self.get_char_id(did).await?;
        Ok(UserData{cid,rid})
    }
    pub async fn many_card(&self,user:i32)->Result<Vec<Card>,PgCustomError>{
        let cid = self.get_all_cid(user).await?;
        let mut card = Vec::new();
        for i in cid{
            card.push(self.get_card(i).await?);
        }
        return Ok(card);
    }
    pub async fn get_card(&self,cid:i32)->Result<Card,PgCustomError>{
        let idk = sqlx::query_as::<_,Card>(
            "SELECT characters.id as char_id, user_id,characters.name as name,gr,hrp,weapon_type,
            characters.last_login as login,username,guild_id,guilds.name as guild_name 
            FROM characters 
            INNER JOIN users ON characters.user_id = users.id 
            LEFT OUTER JOIN guild_characters ON characters.id = guild_characters.character_id
            LEFT OUTER JOIN guilds ON guild_characters.guild_id = guilds.id
            WHERE characters.id=$1").bind(cid).fetch_one(&**self).await?;
        Ok(idk)
    }
}
