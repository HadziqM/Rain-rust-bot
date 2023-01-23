use serenity::model::prelude::ChannelId;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

use crate::{CONFIG, USER};
use super::super::utils::color;



pub async fn error(ctx:&Context,err:&str,on:&str,advice:&str){
    let icon;
    let ch_id;
    unsafe{
        icon = USER.to_owned().unwrap().avatar_url().unwrap();
        ch_id = ChannelId(CONFIG.to_owned().unwrap().log_channel.err_channel.parse::<u64>().unwrap());
    }
    if let Err(why) = ch_id.send_message(&ctx.http, |msg|{
        msg.embed(|emb|{
            emb.title("🛑 Error Occured 🛑")
                .description("some cant be handled error occured")
                .fields(vec![
                    ("🚧 occured on",on,false),
                    ("📜 error message",&format!("```\n{err}\n```"),false),
                    ("⛑  author advice",advice,false)
                ])
                .footer(|f|f.text("you can consult this to HertzIq#0494").icon_url(icon))
                .color(color("ff", "00", "00"))
                .thumbnail("attachment://panics.png")
        }).add_file("./icon/panics.png")
    }).await{
        println!("cant send error message to discord channel :{}",why)
    }
}

pub async fn error_rply(ctx:&Context,err:&str,on:&str,advice:&str,cmd:&ApplicationCommandInteraction){
    let icon;
    unsafe{
        icon = USER.to_owned().unwrap().default_avatar_url();
    }
    if let Err(why) = cmd.create_interaction_response(&ctx.http, |msg|{
        msg.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|m| m.add_file("./icon/panics.svg").embed(|emb|{
            emb.title("Error Occured")
                .description("some cant be handled error occured")
                .fields(vec![
                    ("occured on",on,false),
                    ("error message",&format!("```\n{err}\n```"),false),
                    ("author advice",advice,false)
                ])
                .footer(|f|f.text("you can consult this to HertzIq#0494").icon_url(icon))
                .color(color("ff", "00", "00"))
                .image("attachment://panics.svg")
        }))
    }).await{
        error(ctx, why.to_string().as_str(), "sending error msg", "just discord connection problem").await;
        println!("{why}")
    }
}
