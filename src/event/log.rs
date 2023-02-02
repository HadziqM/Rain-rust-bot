use std::{path::Path, time::SystemTime};

use serenity::{prelude::Context, model::prelude::{UserId, ChannelId}};

use crate::{Init,ErrorLog};

async fn emptying_log(path:&Path)->Result<(),tokio::io::Error>{
    let file = tokio::fs::File::open(path).await?;
    file.set_len(0).await?;
    file.sync_all().await?;
    Ok(())
}

pub async fn logging(ctx:&Context,init:&Init){
    let path = Path::new(".").join("log.txt");
    let user =UserId(init.discord.author_id).to_user(&ctx.http).await.unwrap();
    let mut err = ErrorLog::new(ctx,init,&user).await;
    let channel = ChannelId(init.log_channel.erupe_channel);
    if let Err(why)=channel.send_message(&ctx.http,|m|{
        m.add_file(&path).content(&format!("LOG AT <t:{}:F>",SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()))
    }).await{
        err.change_error(why.to_string(), "sending log", "filed to send log".to_string());
        err.log_error_channel().await;
    };
    if let Err(why)=emptying_log(&path).await{
        err.change_error(why.to_string(), "empetying log", "please do it manually".to_string());
        err.log_error_channel().await;
    }
}
