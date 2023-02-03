use std::{path::Path, time::SystemTime, num::NonZeroU64};

use serenity::{prelude::Context, model::{prelude::{UserId, ChannelId}, user::User}, builder::{CreateMessage, CreateAttachment}};

use crate::{Init,ErrorLog};

async fn emptying_log(path:&Path)->Result<(),tokio::io::Error>{
    tokio::fs::remove_file(path).await?;
    tokio::fs::File::create(path).await?;
    Ok(())
}

pub async fn logging(ctx:&Context,init:&Init,wish:Option<User>){
    let path = Path::new(".").join("log.txt").as_path().to_owned();
    let user =UserId(NonZeroU64::new(init.discord.author_id).unwrap()).to_user(&ctx.http).await.unwrap();
    let mut err = ErrorLog::new(ctx,init,&user).await;
    let channel = ChannelId(NonZeroU64::new(init.log_channel.erupe_channel).unwrap());
    if let Err(why)=channel.send_message(&ctx.http,CreateMessage::new()
        .add_file(CreateAttachment::path(&path).await.unwrap()).content(&format!("LOG AT <t:{}:F>"
        ,SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()))).await{
        err.change_error(why.to_string(), "sending log", "failed to send log".to_string());
        err.log_error_channel().await;
    };
    //send dm to wish if servercrash
    if let Some(wish)=wish{
        if let Err(why)= wish.dm(&ctx.http,CreateMessage::new()
                .content("server crash on about now or few minutes before, please check the log")
                .add_file(CreateAttachment::path(&path).await.unwrap())).await{
            err.change_error(why.to_string(),"dm wish the log File", "please investigate".to_string());
            err.log_error_channel().await;
        }
        if let Err(why)= channel.send_message(&ctx.http, CreateMessage::new().content("SERVER MIGHT CRASH JUST ABOUT NOW")).await{
            err.change_error(why.to_string(), "sending emergency message", "server crash now".to_string());
            err.log_error_channel().await;
        }
    }
    if let Err(why)=emptying_log(&path).await{
        err.change_error(why.to_string(), "empetying log", "please do it manually".to_string());
        err.log_error_channel().await;
    }
}
