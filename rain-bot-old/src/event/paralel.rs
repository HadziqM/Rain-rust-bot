use std::{num::NonZeroU64, path::Path, time::SystemTime};

use crate::reusable::postgress::server::Servers;
use crate::reusable::utils::Color;
use crate::{Init, MyErr, PgConn, MONITOR};
use serenity::all::*;

struct Server<'a> {
    pg: PgConn<'a>,
    msg: Message,
    ctx: &'a Context,
}

impl<'a> Server<'a> {
    async fn new(init: &'a Init, ctx: &'a Context) -> Result<Server<'a>, MyErr> {
        let channel = ChannelId(NonZeroU64::new(init.log_channel.info_channel).unwrap());
        let msg = channel
            .message(&ctx.http, init.log_channel.info_channel_msg)
            .await?;
        let pg = PgConn::create(init, init.discord.author_id.to_string()).await?;
        Ok(Server { pg, msg, ctx })
    }
    fn build_embed(&self, serv: Vec<Servers>) -> CreateEmbed {
        let mut field = Vec::new();
        let mut pc = 0;
        for x in &serv {
            let z = format!(
                "Description :{}\nPlayer_Count: {}\nLand: {}",
                &x.description, x.cp, x.land
            );
            field.push((&x.name, z, true));
            pc += x.cp
        }
        CreateEmbed::new()
            .title("Server Status")
            .description(&format!(
                "Mhfz Server Status updated <t:{}:R>\nPlayer Count Total = {pc}",
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            ))
            .fields(field)
            .color(Color::Random.throw())
    }
    async fn edit_msg(&mut self) -> Result<i32, MyErr> {
        let z = self.pg.get_server().await?;
        let pc = z.iter().map(|e| e.cp).sum::<i32>();
        let embed = self.build_embed(z);
        self.msg
            .edit(&self.ctx.http, EditMessage::new().embed(embed))
            .await?;
        self.pg.close().await;
        Ok(pc)
    }
}

//event handler will spawn a thread calling this function every 1 minutes and log every crash or 10
//minutes
pub async fn handle(ctx: &Context, init: &Init, state: i32, log: bool) -> i32 {
    let mon = MONITOR.lock().await;
    if !*mon {
        return 0;
    }
    match paralel_thread(ctx, init, state, log).await {
        Ok(x) => x,
        Err(_) => 0,
    }
}
async fn paralel_thread(ctx: &Context, init: &Init, state: i32, log: bool) -> Result<i32, MyErr> {
    let mut serv = Server::new(init, ctx).await?;
    //run function to update server status on info channel
    let now = serv.edit_msg().await?;
    let crash = now == 0 && state != 0;
    if log || crash {
        let wish;
        if crash {
            wish = Some(
                UserId(NonZeroU64::new(119094696487288833).unwrap())
                    .to_user(&ctx.http)
                    .await
                    .unwrap(),
            );
        } else {
            wish = None
        }
        //execute the logging logic
        logging(ctx, init, wish).await?;
    }
    Ok(now)
}
async fn emptying_log(path: &Path) -> Result<(), tokio::io::Error> {
    tokio::fs::remove_file(path).await?;
    tokio::fs::File::create(path).await?;
    Ok(())
}

async fn announce_crash(channel: &ChannelId, ctx: &Context, init: &Init) -> Result<(), MyErr> {
    let maintainer = format!("<@&{}>", init.server_role.maintainer_role);
    channel
        .send_message(
            &ctx.http,
            CreateMessage::new()
                .content(format!("SERVER MIGHT CRASH JUST ABOUT NOW {}", maintainer)),
        )
        .await?;
    Ok(())
}
async fn logging(ctx: &Context, init: &Init, wish: Option<User>) -> Result<(), MyErr> {
    let path = Path::new(".").join("log.txt").as_path().to_owned();
    let channel = ChannelId(NonZeroU64::new(init.log_channel.erupe_channel).unwrap());
    if !init.mhfz_config.sending_log {
        if wish.is_some() {
            announce_crash(&channel, ctx, init).await?;
        }
        return Ok(());
    }
    let attachment = CreateAttachment::path(&path).await?;
    channel
        .send_message(
            &ctx.http,
            CreateMessage::new().add_file(attachment).content(&format!(
                "LOG AT <t:{}:F>",
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            )),
        )
        .await?;
    //send dm to wish if servercrash
    if let Some(wish) = wish {
        announce_crash(&channel, ctx, init).await?;
        wish.dm(
            &ctx.http,
            CreateMessage::new()
                .content("server crash on about now or few minutes before, please check the log")
                .add_file(CreateAttachment::path(&path).await.unwrap()),
        )
        .await?;
    }
    emptying_log(&path).await?;
    Ok(())
}
