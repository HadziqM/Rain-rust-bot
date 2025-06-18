#![allow(dead_code)]

use crate::MyResult;
use crate::{item_code::ItemCode, SYSDIR};
use log::debug;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use strum::{EnumIter, EnumString};

pub trait JsonSetting: Serialize + DeserializeOwned {
    fn open(ty: SettingList) -> Result<Self, Box<dyn std::error::Error>> {
        let path = SYSDIR.find_path(ty.path()).ok_or("config path not found")?;
        debug!("loading setting file {path}");
        Ok(serde_json::from_slice(&std::fs::read(path)?)?)
    }

    fn placeholder(ty: SettingList) -> Result<(), Box<dyn std::error::Error>>
    where
        Self: Default,
    {
        debug!("creating placeholder file {}", ty.path());
        let pretty = serde_json::to_string_pretty(&Self::default())?;
        std::fs::write(ty.path(), pretty.as_bytes())?;
        Ok(())
    }
}

impl JsonSetting for SettingMain {}
impl JsonSetting for SettingMarket {}
impl JsonSetting for SettingGacha {}
impl JsonSetting for SettingSaveFile {}
impl JsonSetting for SettingDiscord {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SettingAll {
    pub main: SettingMain,
    pub market: SettingMarket,
    pub gacha: SettingGacha,
    pub savefile: SettingSaveFile,
    pub discord: SettingDiscord,
}

impl SettingAll {
    pub fn load_all() -> Self {
        Self {
            main: SettingList::Main.load().unwrap_log(),
            market: SettingList::Market.load().unwrap_log(),
            gacha: SettingList::Gacha.load().unwrap_log(),
            savefile: SettingList::SaveFile.load().unwrap_log(),
            discord: SettingList::Discord.load().unwrap_log(),
        }
    }
    fn create_placeholders() {
        SettingList::Main.placeholder::<SettingMain>().unwrap_log();
        SettingList::Market
            .placeholder::<SettingMarket>()
            .unwrap_log();
        SettingList::Gacha
            .placeholder::<SettingGacha>()
            .unwrap_log();
        SettingList::SaveFile
            .placeholder::<SettingSaveFile>()
            .unwrap_log();
        SettingList::Discord
            .placeholder::<SettingDiscord>()
            .unwrap_log();
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, EnumIter, EnumString, strum::Display)]
pub enum SettingList {
    Main,
    Discord,
    SaveFile,
    Gacha,
    Market,
}

impl SettingList {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Main => "main.json",
            Self::Discord => "discord.json",
            Self::SaveFile => "savefile.json",
            Self::Gacha => "gacha.json",
            Self::Market => "market.json",
        }
    }

    pub fn validete_and_change(
        &self,
        bytes: &[u8],
        setting: &mut SettingAll,
    ) -> Result<(), String> {
        match self {
            SettingList::Main => {
                let x = serde_json::from_slice::<SettingMain>(bytes)
                    .map_err(|e| format!("Validation failed for Main: {e}"))?;
                setting.main = x;
            }
            SettingList::Discord => {
                let x = serde_json::from_slice::<SettingDiscord>(bytes)
                    .map_err(|e| format!("Validation failed for Main: {e}"))?;
                setting.discord = x;
            }
            SettingList::SaveFile => {
                let x = serde_json::from_slice::<SettingSaveFile>(bytes)
                    .map_err(|e| format!("Validation failed for Main: {e}"))?;
                setting.savefile = x;
            }
            SettingList::Gacha => {
                let x = serde_json::from_slice::<SettingGacha>(bytes)
                    .map_err(|e| format!("Validation failed for Main: {e}"))?;
                setting.gacha = x;
            }
            SettingList::Market => {
                let x = serde_json::from_slice::<SettingMarket>(bytes)
                    .map_err(|e| format!("Validation failed for Main: {e}"))?;
                setting.market = x;
            }
        };
        Ok(())
    }

    fn placeholder<T: JsonSetting + Default>(self) -> Result<(), Box<dyn std::error::Error>> {
        T::placeholder(self)
    }
    pub fn load<T: JsonSetting>(self) -> Result<T, Box<dyn std::error::Error>> {
        T::open(self)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SettingGacha {
    pub cost: u32,
    pub pity: u32,
    pub multi_count: u32,
    pub rarity: GachaRaritySetting,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GachaRaritySetting {
    pub ur: Vec<ItemCode>,
    pub ssr1: Vec<ItemCode>,
    pub ssr2: Vec<ItemCode>,
    pub sr1: Vec<ItemCode>,
    pub sr2: Vec<ItemCode>,
    pub sr3: Vec<ItemCode>,
    pub r1: Vec<ItemCode>,
    pub r2: Vec<ItemCode>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SettingSaveFile {
    pub cooldown_hour: u32,
    pub autoaccept_countdown_mins: i32,
    pub allowed_file: AllowedFileSetting,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AllowedFileSetting {
    pub savedata: bool,
    pub decomyset: bool,
    pub hunternavi: bool,
    pub otomoairou: bool,
    pub partner: bool,
    pub platedata: bool,
    pub platebox: bool,
    pub platemyset: bool,
    pub rengokudata: bool,
    pub savemercenary: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SettingMarket {
    pub market: Vec<ItemCode>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SettingMain {
    pub discord: DiscordBotSetting,
    pub database: DatabaseSetting,
    pub updater: GithubUpdaterSetting,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SettingDiscord {
    pub channel: DiscordChannelSetting,
    pub role: DiscordServerRole,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DiscordChannelSetting {
    pub log_channel: u64,
    pub error_channel: u64,
    pub transfer_channel: u64,
    pub bounty_submission: u64,
    pub bounty_title: u64,
    pub speedrun_submission: u64,
    pub speedrun_leaderboard_channel: u64,
    pub speedrun_leaderboard_msg: u64,
    pub market_channel: u64,
    pub market_menu_channel: u64,
    pub market_menu_msg: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DiscordServerRole {
    pub admin: u64,
    pub registered: u64,
    pub save_judge: u64,
    pub speedrun_judge: u64,
    pub bounty_judge: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DiscordBotSetting {
    pub token: String,
    pub webhook: String,
    pub author: u64,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DatabaseSetting {
    pub user: String,
    pub host: String,
    pub password: String,
    pub port: u16,
    pub database: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GithubUpdaterSetting {
    pub repo: String,
    pub owner: String,
    pub token: Option<String>,
    pub app_name: String,
}

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn create_placeholders() {
//         SettingList::Main.path().execute_dir();
//         println!("{:?}", SettingList::Main.path());
//         SettingAll::create_placeholders();
//     }
// }
