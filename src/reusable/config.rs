use std::fs::read_to_string;
use serde::Deserialize;
use std::error::Error;


#[derive(Debug,Deserialize,Clone)]
pub struct Init {
    pub(crate) server_market: ServerMarket,
    pub(crate) speedrun_contest: SpeedrunContest,
    pub(crate) pvp_contest: PvpContest,
    pub(crate) mezfes_contest: MezfesContest,
    pub(crate) transmog_contest: TransmogContest,
    pub(crate) gacha_channel: GachaChannel,
    pub(crate) bounty_message_id:BountyMessageId,
    pub(crate) bounty_channel:BountyChannel,
    pub(crate) server_role:ServerRole,
    pub(crate) server_channel_url:ServerChannelUrl,
    pub(crate) server_channel:ServerChannel,
    pub(crate) log_channel:LogChannels,
    pub(crate) bot_config:BotConfig,
    pub(crate) mhfz_config:MhfzConfig,
    pub(crate) postgress:Postgress,
    pub(crate) discord:Discord
}
#[derive(Debug,Deserialize,Clone)]
pub struct Discord {
    pub(crate) token: String,
    pub(crate) prefix: String,
    pub(crate) author_id:u64
}
#[derive(Debug,Deserialize,Clone)]
pub struct Postgress {
    pub(crate) host: String,
    pub(crate) password: String,
    pub(crate) port: u16,
    pub(crate) database: String,
}
#[derive(Debug,Deserialize,Clone)]
pub struct MhfzConfig {
    pub(crate) account_creation: bool
}
#[derive(Debug,Deserialize,Clone)]
pub struct BotConfig {
    pub(crate) member_join: bool,
    pub(crate) member_leave: bool,
    pub(crate) role_moderation: bool,
    pub(crate) member_moderation: bool,
    pub(crate) gacha: bool,
    pub(crate) bounty: bool,
    pub(crate) transmog_contest: bool,
    pub(crate) mezfes_contest: bool,
    pub(crate) server_market: bool,
    pub(crate) pvp_contest: bool,
    pub(crate) speedrun_contest: bool,
}
#[derive(Debug,Deserialize,Clone)]
pub struct LogChannels {
    pub(crate) err_channel: u64,
    pub(crate) account_channel: u64,
    pub(crate) transfer_channel: u64,
    pub(crate) moderation_channel: u64,
}

#[derive(Debug,Deserialize,Clone)]
pub struct ServerChannel {
    pub(crate) member_join: u64,
    pub(crate) member_leave: u64,
    pub(crate) rule_channel: u64,
    pub(crate) rule_msg_id: u64,
}
#[derive(Debug,Deserialize,Clone)]
pub struct ServerChannelUrl {
    pub(crate) guide_channel: u64,
    pub(crate) game_channel: u64,
    pub(crate) bot_channel: u64,
}
#[derive(Debug,Deserialize,Clone)]
pub struct ServerRole {
    pub(crate) admin_role: u64,
    pub(crate) member_role: u64,
    pub(crate) mute_role: u64,
}
#[derive(Debug,Deserialize,Clone)]
pub struct BountyChannel {
    pub(crate) board: u64,
    pub(crate) conquered: u64,
    pub(crate) promotion: u64,
    pub(crate) cooldown_ch: u64,
    pub(crate) judge_ch: u64,
}
#[derive(Debug,Deserialize,Clone)]
pub struct BountyMessageId {
    pub(crate) cooldown_msg: u64,
    pub(crate) leaderboard_msg: u64,
}
#[derive(Debug,Deserialize,Clone)]
pub struct GachaChannel {
    pub(crate) pull: u64,
}
#[derive(Debug,Deserialize,Clone)]
pub struct TransmogContest{
    pub(crate) submitted_channel: u64,
}
#[derive(Debug,Deserialize,Clone)]
pub struct MezfesContest{
    pub(crate) leaderboard_channel: u64,
    pub(crate) leaderboard_msg_id: u64,
}
#[derive(Debug,Deserialize,Clone)]
pub struct PvpContest{
    pub(crate) leaderboard_channel: u64,
    pub(crate) leaderboard_msg_id: u64,
}
#[derive(Debug,Deserialize,Clone)]
pub struct SpeedrunContest{
    pub(crate) leaderboard_channel: u64,
    pub(crate) leaderboard_msg_id: u64,
}
#[derive(Debug,Deserialize,Clone)]
pub struct ServerMarket{
    pub(crate) market_channel: u64,
}

impl Init {
    pub fn default()->Init{
                Init { server_market: ServerMarket { market_channel: 0 },
            speedrun_contest: SpeedrunContest { leaderboard_channel: 0, leaderboard_msg_id:0 },
            pvp_contest: PvpContest { leaderboard_channel: 0, leaderboard_msg_id: 0 },
            mezfes_contest: MezfesContest { leaderboard_channel: 0, leaderboard_msg_id: 0 },
            transmog_contest: TransmogContest { submitted_channel: 0 },
            gacha_channel: GachaChannel { pull: 0 },
            bounty_message_id: BountyMessageId { cooldown_msg: 0, leaderboard_msg: 0 },
            bounty_channel: BountyChannel { board: 0,
                conquered: 0,
                promotion: 0,
                cooldown_ch: 0,
                judge_ch: 0},
            server_role: ServerRole { admin_role: 0, member_role: 0, mute_role: 0 },
            server_channel_url: ServerChannelUrl { guide_channel: 0,
                game_channel: 0,
                bot_channel: 0 },
            server_channel: ServerChannel { member_join: 0,
                member_leave: 0,
                rule_channel: 0,
                rule_msg_id: 0 },
            log_channel: LogChannels { err_channel: 0,
                account_channel: 0, 
                transfer_channel: 0,
                moderation_channel: 0 },
            bot_config: BotConfig { member_join: true,
                member_leave: true,
                role_moderation: true,
                member_moderation: true,
                gacha: true,
                bounty: true,
                transmog_contest: true,
                mezfes_contest: true,
                server_market: true,
                pvp_contest: true,
                speedrun_contest: true },
            mhfz_config: MhfzConfig { account_creation: true },
            postgress: Postgress { host: String::new(),
                password: String::new(),
                port: 0,
                database: String::new() },
            discord: Discord { 
                token: String::new(), 
                prefix: String::new(),
                author_id:0
            }
     }
    }
}

pub fn get_config()->Result<Init,Box<dyn Error>>{
    let input = read_to_string("./config.json")?;
    Ok(serde_json::from_str(&input)?)
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn config() {
        let idk = get_config().unwrap();
        assert_eq!(idk.discord.prefix,"%".to_string());
    }
}
