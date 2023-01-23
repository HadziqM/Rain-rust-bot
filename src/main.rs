#![allow(dead_code)]

pub mod reusable;
pub mod commands;
mod event;

use reusable::config::*;
use serenity::prelude::*;
use event::Handler;

pub static mut CONFIG:Init = 
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
     };
#[tokio::main]
async fn main() {
    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    match get_config(){
        Ok(conf)=> {
            unsafe{
                CONFIG = conf.clone();
            }
            let mut client = Client::builder(conf.discord.token, intents)
                .event_handler(Handler)
                .await
                .expect("Error creating client");
            if let Err(why) = client.start().await {
                println!("Client error: {:?}", why);
            }
        }
        Err(why)=>{
            println!("serialize error: {}",why)
        }
    }
}
