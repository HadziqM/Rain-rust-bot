#[derive(Debug, Clone)]
pub struct DbCard {
    pub char_id: Option<i32>,
    pub user_id: Option<i64>,
    pub name: Option<String>,
    pub gr: Option<i32>,
    pub hrp: Option<i32>,
    pub login: Option<i32>,
    pub weapon_type: Option<i32>,
    pub username: String,
    pub guild_id: Option<i32>,
    pub guild_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DbEvent {
    pub bounty: i32,
    pub gacha: i32,
    pub pity: i32,
    pub latest_bounty: String,
    pub latest_bounty_time: i64,
    pub title: Option<i32>,
    pub bronze: Option<i32>,
    pub silver: Option<i32>,
    pub gold: Option<i32>,
    pub name: Option<String>,
    pub char_id: i32,
}

#[derive(Debug, Clone)]
pub struct DbUserData {
    // Character_id
    pub cid: Option<i32>,
    // User_id
    pub uid: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct DbAccountData {
    pub id: i32,
    pub username: String,
    pub password: String,
}
#[derive(Debug, Clone)]
pub struct DbSaveData {
    pub savedata: Option<Vec<u8>>,
    pub decomyset: Option<Vec<u8>>,
    pub hunternavi: Option<Vec<u8>>,
    pub otomoairou: Option<Vec<u8>>,
    pub partner: Option<Vec<u8>>,
    pub platedata: Option<Vec<u8>>,
    pub platebox: Option<Vec<u8>>,
    pub platemyset: Option<Vec<u8>>,
    pub rengokudata: Option<Vec<u8>>,
    pub savemercenary: Option<Vec<u8>>,
}

impl DbCard {
    pub fn get_path(&self) -> String {
        let iconlist = vec![
            "https://media.discordapp.net/attachments/1068440173479739393/1068440322977312868/GS.png",
            "https://media.discordapp.net/attachments/1068440173479739393/1068440324617281626/HS.png",
            "https://media.discordapp.net/attachments/1068440173479739393/1068440323501596792/H.png",
            "https://media.discordapp.net/attachments/1068440173479739393/1068440324931862599/L.png",
            "https://media.discordapp.net/attachments/1068440173479739393/1068440373548044348/SS.png",
            "https://media.discordapp.net/attachments/1068440173479739393/1068440325309345822/LB.png",
            "https://media.discordapp.net/attachments/1068440173479739393/1068440322260086794/DS.png",
            "https://media.discordapp.net/attachments/1068440173479739393/1068440372474302464/LS.png",
            "https://media.discordapp.net/attachments/1068440173479739393/1068440324088807466/HH.png",
            "https://media.discordapp.net/attachments/1068440173479739393/1068440322633383946/GL.png",
            "https://media.discordapp.net/attachments/1068440173479739393/1068440321907761162/B.png",
            "https://media.discordapp.net/attachments/1068440173479739393/1068440373757743154/T.png",
            "https://media.discordapp.net/attachments/1068440173479739393/1068440373132800080/SAF.png",
            "https://media.discordapp.net/attachments/1068440173479739393/1068440372709167174/MS.png",
        ];
        iconlist[self.weapon_type.unwrap_or_default() as usize].to_string()
    }
    pub fn g_name(&self) -> String {
        match &self.guild_name {
            Some(x) => x.to_owned(),
            None => "No guild".to_string(),
        }
    }
    pub fn g_id(&self) -> String {
        match self.guild_id {
            Some(x) => x.to_string(),
            None => "No id".to_string(),
        }
    }
    pub fn hrp(&self) -> u8 {
        match self.hrp.unwrap_or(0) {
            999 => 7,
            300..=998 => 6,
            100..=299 => 5,
            51..=99 => 4,
            31..=50 => 3,
            1..=30 => 2,
            _ => 1,
        }
    }
    pub fn last_login(&self) -> String {
        format!("<t:{}:R>", self.login.unwrap_or_default())
    }
}
