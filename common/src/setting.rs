#![allow(dead_code)]

use crate::MyResult;
use crate::{item_code::ItemCode, SYSDIR};
use log::debug;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sysdir::Sysdir;

pub trait JsonSetting: Serialize + DeserializeOwned {
    fn open(ty: SettingList) -> Result<Self, Box<dyn std::error::Error>> {
        debug!("loading setting file {}", ty.path());
        Ok(serde_json::from_slice(&std::fs::read(ty.path())?)?)
    }
}

impl JsonSetting for SettingMain {}
impl JsonSetting for SettingMarket {}
impl JsonSetting for SettingGacha {}
impl JsonSetting for SettingSaveFile {}
impl JsonSetting for SettingDiscord {}

#[derive(Serialize, Deserialize, Clone)]
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
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum SettingList {
    Main,
    Discord,
    SaveFile,
    Gacha,
    Market,
}

impl SettingList {
    pub fn path(&self) -> Sysdir {
        match self {
            Self::Main => SYSDIR.config_dir("main.json"),
            Self::Discord => SYSDIR.config_dir("discord.json"),
            Self::SaveFile => SYSDIR.config_dir("savefile.json"),
            Self::Gacha => SYSDIR.config_dir("gacha.json"),
            Self::Market => SYSDIR.config_dir("market.json"),
        }
    }
    pub fn load<T: JsonSetting>(self) -> Result<T, Box<dyn std::error::Error>> {
        T::open(self)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SettingGacha {
    pub cost: u32,
    pub pity: u32,
    pub rarity: GachaRaritySetting,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GachaRaritySetting {
    pub ur: Vec<ItemCode>,
    pub ssr1: Vec<ItemCode>,
    pub ssr2: Vec<ItemCode>,
    pub sr1: Vec<ItemCode>,
    pub sr2: Vec<ItemCode>,
    pub r1: Vec<ItemCode>,
    pub r2: Vec<ItemCode>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SettingSaveFile {
    pub cooldown_hour: u32,
    pub autoaccept_countdown_mins: i32,
    pub allowed_file: AllowedFileSetting,
}

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
pub struct SettingMarket {
    pub market: Vec<ItemCode>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SettingMain {
    pub discord: DiscordBotSetting,
    pub database: DatabaseSetting,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SettingDiscord {
    pub channel: DiscordChannelSetting,
    pub role: DiscordServerRole,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DiscordChannelSetting {
    pub log_channel: String,
    pub transfer_channel: String,
    pub bounty_submision: String,
    pub bounty_title: String,
    pub speedrun_submision: String,
    pub speedrun_leaderboard_channel: String,
    pub speedrun_leaderboard_msg: String,
    pub market_channel: String,
    pub market_menu_channel: String,
    pub market_menu_msg: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DiscordServerRole {
    pub admin: String,
    pub registered: String,
    pub save_judge: String,
    pub speedrun_judge: String,
    pub bounty_judge: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DiscordBotSetting {
    pub token: String,
    pub webhook: String,
    pub author: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct DatabaseSetting {
    pub user: String,
    pub host: String,
    pub password: String,
    pub port: u16,
    pub database: String,
}
