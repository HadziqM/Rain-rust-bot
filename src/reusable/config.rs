use std::{fs::read_to_string};
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
    pub(crate) err_channel: String,
    pub(crate) account_channel: String,
    pub(crate) transfer_channel: String,
    pub(crate) moderation_channel: String,
}

#[derive(Debug,Deserialize,Clone)]
pub struct ServerChannel {
    pub(crate) member_join: String,
    pub(crate) member_leave: String,
    pub(crate) rule_channel: String,
    pub(crate) rule_msg_id: String,
}
#[derive(Debug,Deserialize,Clone)]
pub struct ServerChannelUrl {
    pub(crate) guide_channel: String,
    pub(crate) game_channel: String,
    pub(crate) bot_channel: String,
}
#[derive(Debug,Deserialize,Clone)]
pub struct ServerRole {
    pub(crate) admin_role: String,
    pub(crate) member_role: String,
    pub(crate) mute_role: String,
}
#[derive(Debug,Deserialize,Clone)]
pub struct BountyChannel {
    pub(crate) board: String,
    pub(crate) conquered: String,
    pub(crate) promotion: String,
    pub(crate) cooldown_ch: String,
    pub(crate) judge_ch: String,
}
#[derive(Debug,Deserialize,Clone)]
pub struct BountyMessageId {
    pub(crate) cooldown_msg: String,
    pub(crate) leaderboard_msg: String,
}
#[derive(Debug,Deserialize,Clone)]
pub struct GachaChannel {
    pub(crate) pull: String,
}
#[derive(Debug,Deserialize,Clone)]
pub struct TransmogContest{
    pub(crate) submitted_channel: String,
}
#[derive(Debug,Deserialize,Clone)]
pub struct MezfesContest{
    pub(crate) leaderboard_channel: String,
    pub(crate) leaderboard_msg_id: String,
}
#[derive(Debug,Deserialize,Clone)]
pub struct PvpContest{
    pub(crate) leaderboard_channel: String,
    pub(crate) leaderboard_msg_id: String,
}
#[derive(Debug,Deserialize,Clone)]
pub struct SpeedrunContest{
    pub(crate) leaderboard_channel: String,
    pub(crate) leaderboard_msg_id: String,
}
#[derive(Debug,Deserialize,Clone)]
pub struct ServerMarket{
    pub(crate) market_channel: String,
}

impl Init {
    pub fn default()->Init{
                Init { server_market: ServerMarket { market_channel: String::new() },
            speedrun_contest: SpeedrunContest { leaderboard_channel: String::new(), leaderboard_msg_id:String::new() },
            pvp_contest: PvpContest { leaderboard_channel: String::new(), leaderboard_msg_id: String::new() },
            mezfes_contest: MezfesContest { leaderboard_channel: String::new(), leaderboard_msg_id: String::new() },
            transmog_contest: TransmogContest { submitted_channel: String::new() },
            gacha_channel: GachaChannel { pull: String::new() },
            bounty_message_id: BountyMessageId { cooldown_msg: String::new(), leaderboard_msg: String::new() },
            bounty_channel: BountyChannel { board: String::new(),
                conquered: String::new(),
                promotion: String::new(),
                cooldown_ch: String::new(),
                judge_ch: String::new()},
            server_role: ServerRole { admin_role: String::new(), member_role: String::new(), mute_role: String::new() },
            server_channel_url: ServerChannelUrl { guide_channel: String::new(),
                game_channel: String::new(),
                bot_channel: String::new() },
            server_channel: ServerChannel { member_join: String::new(),
                member_leave: String::new(),
                rule_channel: String::new(),
                rule_msg_id: String::new() },
            log_channel: LogChannels { err_channel: String::new(),
                account_channel: String::new(), 
                transfer_channel: String::new(),
                moderation_channel: String::new() },
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
            discord: Discord { token: String::new(), prefix: String::new() }
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
