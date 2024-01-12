use crate::{commands, Init, ItemPedia, MyErr};
use serenity::all::*;

pub struct MsgBundle<'a> {
    pub ctx: &'a Context,
    pub msg: &'a Message,
    pub init: &'a Init,
    pub pedia: &'a ItemPedia,
}

pub async fn msg_handler(ctx: &Context, msg: &Message, init: &Init, pedia: &ItemPedia) {
    if msg.content.starts_with(&init.discord.prefix) && !msg.author.bot {
        let name = msg.content.split_whitespace().next();
        let bnd = MsgBundle {
            ctx,
            msg,
            init,
            pedia,
        };
        if let Some(x) = name {
            let y = x.replace(&init.discord.prefix, "");
            match y.as_str() {
                "test" => discord_test(&bnd).await,
                "ping" => discord_ping(&bnd).await,
                "execute" => commands::admin::query::discord_msg(&bnd).await,
                "query" => commands::admin::query::discord_qry(&bnd).await,
                "tag" => commands::misc::tag::discord_message(&bnd).await,
                _ => {
                    return;
                }
            }
        }
    }
}
#[hertz::hertz_msg(false)]
async fn test(bnd: &MsgBundle<'_>) -> Result<(), MyErr> {
    bnd.msg
        .channel_id
        .send_message(
            &bnd.ctx.http,
            CreateMessage::new().embed(CreateEmbed::new().title("tested")),
        )
        .await?;
    Ok(())
}
#[hertz::hertz_msg(false)]
async fn ping(bnd: &MsgBundle<'_>) -> Result<(), MyErr> {
    let now = std::time::Instant::now();
    use crate::Components;
    let mut msg = Components::msg(bnd, "pong").await?;
    let discord_ping = format!("discord connection = {:.2?}", now.elapsed());
    let pg = std::time::Instant::now();
    let mut post = crate::PgConn::create(bnd.init, bnd.init.discord.author_id.to_string()).await?;
    let pg_ping = format!("postgres connection = {:.2?}", pg.elapsed());
    let fs = std::time::Instant::now();
    let _init = Init::new().await?;
    let fs_ping = format!("json filesystem speed = {:.2?}", fs.elapsed());
    let embed = CreateEmbed::new()
        .title("ping")
        .field("discord", discord_ping, false)
        .field("postgres", pg_ping, false)
        .field("Json filesystem", fs_ping, false)
        .color(crate::reusable::utils::Color::Random.throw());
    msg.edit(&bnd.ctx.http, EditMessage::new().embed(embed))
        .await?;
    post.close().await;
    Ok(())
}
