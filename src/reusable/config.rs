use std::path::{Path, PathBuf};
use serde::{Deserialize,Serialize};
use crate::MyErr;


#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct Init {
    // pub(crate) server_market: ServerMarket,
    // pub(crate) speedrun_contest: SpeedrunContest,
    // pub(crate) pvp_contest: PvpContest,
    // pub(crate) mezfes_contest: MezfesContest,
    // pub(crate) transmog_contest: TransmogContest,
    // pub(crate) gacha_channel: GachaChannel,
    // pub(crate) bounty_message_id:BountyMessageId,
    pub(crate) bounty:Bounty,
    pub chat_gpt:ChatGpt,
    pub(crate) server_role:ServerRole,
    // pub(crate) server_channel_url:ServerChannelUrl,
    // pub(crate) server_channel:ServerChannel,
    pub(crate) log_channel:LogChannels,
    pub(crate) bot_config:BotConfig,
    pub(crate) mhfz_config:MhfzConfig,
    pub(crate) postgress:Postgress,
    pub(crate) discord:Discord
}
#[derive(Debug,Deserialize,Clone,Serialize)]
pub struct Discord {
    pub(crate) token: String,
    pub(crate) prefix: String,
    pub(crate) author_id:u64
}
#[derive(Debug,Deserialize,Clone,Serialize)]
pub struct Postgress {
    pub(crate) host: String,
    pub(crate) password: String,
    pub(crate) port: u16,
    pub(crate) database: String,
    pub(crate) user:String
}
#[derive(Debug,Deserialize,Clone,Serialize)]
pub struct MhfzConfig {
    pub(crate) account_creation: bool,
    pub(crate) sending_log: bool
}
#[derive(Debug,Deserialize,Clone,Serialize)]
pub struct BotConfig {
    pub(crate) member_join: bool,
    pub(crate) member_leave: bool,
    // pub(crate) role_moderation: bool,
    // pub(crate) member_moderation: bool,
    pub(crate) gacha: bool,
    pub(crate) bounty: bool,
    // pub(crate) transmog_contest: bool,
    // pub(crate) mezfes_contest: bool,
    pub(crate) server_market: bool,
    // pub(crate) pvp_contest: bool,
    // pub(crate) speedrun_contest: bool,
}
#[derive(Debug,Deserialize,Clone,Serialize)]
pub struct LogChannels {
    pub(crate) err_channel: u64,
    pub(crate) account_channel: u64,
    pub(crate) transfer_channel: u64,
    pub(crate) erupe_channel: u64,
    pub(crate) info_channel: u64,
    pub(crate) info_channel_msg: u64,
    pub(crate) market_channel: u64,
    pub(crate) join_channel: u64,
    pub(crate) leave_channel: u64,
    pub(crate) market_channel_msg: u64,
    pub(crate) request_channel: u64,
    // pub(crate) moderation_channel: u64,
}

// #[derive(Debug,Deserialize,Clone)]
// pub struct ServerChannel {
//     pub(crate) member_join: u64,
//     pub(crate) member_leave: u64,
//     pub(crate) rule_channel: u64,
//     pub(crate) rule_msg_id: u64,
// }
// #[derive(Debug,Deserialize,Clone)]
// pub struct ServerChannelUrl {
//     pub(crate) guide_channel: u64,
//     pub(crate) game_channel: u64,
//     pub(crate) bot_channel: u64,
// }
#[derive(Debug,Deserialize,Clone,Serialize)]
pub struct ServerRole {
    pub(crate) admin_role: u64,
    pub(crate) bartender_role: u64,
    pub(crate) member_role: u64,
    // pub(crate) mute_role: u64,
    pub(crate) register_role: u64,
    pub(crate) judge_role: u64,
    pub(crate) maintainer_role: u64,
}
#[derive(Debug,Deserialize,Clone,Serialize)]
pub struct Bounty {
    pub(crate) board_ch: u64,
    pub(crate) conquered_ch: u64,
    pub(crate) receptionist_ch: u64,
    pub(crate) promotion_ch: u64,
    pub(crate) cooldown_ch: u64,
    pub(crate) cooldown_msg: u64,
    pub(crate) judge_ch: u64,
}
#[derive(Debug,Deserialize,Clone,Serialize)]
pub struct ChatGpt {
    pub(crate) token:String,
    pub(crate) timeout:u32
}
// #[derive(Debug,Deserialize,Clone)]
// pub struct BountyMessageId {
//     pub(crate) cooldown_msg: u64,
//     pub(crate) leaderboard_msg: u64,
// }
// #[derive(Debug,Deserialize,Clone)]
// pub struct GachaChannel {
//     pub(crate) pull: u64,
// }
// #[derive(Debug,Deserialize,Clone)]
// pub struct TransmogContest{
//     pub(crate) submitted_channel: u64,
// }
// #[derive(Debug,Deserialize,Clone)]
// pub struct MezfesContest{
//     pub(crate) leaderboard_channel: u64,
//     pub(crate) leaderboard_msg_id: u64,
// }
// #[derive(Debug,Deserialize,Clone)]
// pub struct PvpContest{
//     pub(crate) leaderboard_channel: u64,
//     pub(crate) leaderboard_msg_id: u64,
// }
// #[derive(Debug,Deserialize,Clone)]
// pub struct SpeedrunContest{
//     pub(crate) leaderboard_channel: u64,
//     pub(crate) leaderboard_msg_id: u64,
// }
// #[derive(Debug,Deserialize,Clone)]
// pub struct ServerMarket{
//     pub(crate) market_channel: u64,
// }

impl Init {
    fn path()->PathBuf{
        Path::new(".").join("config.json")
    }
    pub async fn save(&self)->Result<(),MyErr>{
        Ok(tokio::fs::write(&Init::path(), serde_json::to_string_pretty(&self)?).await?)
    }
    pub async fn new()->Result<Init,MyErr>{
        let input = tokio::fs::read_to_string(&Init::path()).await?;
        Ok(serde_json::from_str(&input)?)
    }
    pub fn block_new()->Result<Init,MyErr>{
        let input = std::fs::read_to_string(&Init::path())?;
        Ok(serde_json::from_str(&input)?)
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn config() {
        Init::block_new().unwrap();
    }
}
