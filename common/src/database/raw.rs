pub struct DbCard {
    pub char_id: i32,
    pub user_id: i64,
    pub name: String,
    pub gr: i32,
    pub hrp: i32,
    pub login: i32,
    pub weapon_type: i32,
    pub username: String,
    pub guild_id: Option<i64>,
    pub guild_name: Option<String>,
}

pub struct DbEvent {
    pub bounty: i32,
    pub gacha: i32,
    pub pity: i32,
    pub latest_bounty: String,
    pub latest_bounty_time: i64,
    pub title: i32,
    pub bronze: i32,
    pub silver: i32,
    pub gold: i32,
    pub name: String,
    pub char_id: i32,
}

pub struct DbUserData {
    // Character_id
    pub cid: i32,
    // Account_id
    pub aid: i32,
}

pub struct DbAccountData {
    pub id: i32,
    pub username: String,
    pub password: String,
}
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
        "https://media.discordapp.net/attachments/1068440173479739393/1068440372709167174/MS.png"
        ];
        iconlist[self.weapon_type as usize].to_string()
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
        if self.hrp == 999 {
            return 7;
        } else if self.hrp > 299 {
            return 6;
        } else if self.hrp > 99 {
            return 5;
        } else if self.hrp > 50 {
            return 4;
        } else if self.hrp > 30 {
            return 3;
        } else if self.hrp > 1 {
            return 2;
        }
        1
    }
    pub fn last_login(&self) -> String {
        format!("<t:{}:R>", self.login)
    }
}
