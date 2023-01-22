use serenity::model::prelude::ChannelId;
use serenity::prelude::Context;

use crate::CONFIG;
use super::super::utils::color;



pub async fn error(ctx:&Context,err:&str,on:&str,advice:&str){
    let ch_id;
    unsafe{
        ch_id = ChannelId(CONFIG.err_channel.parse::<u64>().unwrap());
    }
    if let Err(why) = ch_id.send_message(&ctx.http, |msg|{
        msg.embed(|emb|{
            emb.title("error occured")
                .description("some cant be handled error occured")
                .fields(vec![
                    ("occured on",on,true),
                    ("error message",err,true),
                    ("author advice",advice,true)
                ])
                .footer(|f|f.text("you can consult this to HertzIq#0494"))
                .color(color("ff", "00", "00"))
        })
    }).await{
        println!("cant send error message to discord channel :{}",why)
    }
}
