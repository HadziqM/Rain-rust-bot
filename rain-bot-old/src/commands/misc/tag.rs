use crate::reusable::component::market::Tag;
use crate::reusable::utils::dumb_matching;
use crate::{Components, MsgBundle, MyErr};
use serenity::all::*;

#[hertz::hertz_msg(false)]
async fn message(bnd: &MsgBundle<'_>) -> Result<(), MyErr> {
    let tag = Tag::new().await?;
    let arg = bnd
        .msg
        .content
        .split(" ")
        .nth(1)
        .ok_or(MyErr::Custom("cant get message argument".to_owned()))?;
    let name: Vec<_> = tag.tag.iter().map(|x| x.command.to_owned()).collect();
    let mut embed = CreateEmbed::new().color(Color::DARK_GREEN).footer(
        CreateEmbedFooter::new(format!("requested by {}", &bnd.msg.author.name))
            .icon_url(bnd.msg.author.face()),
    );
    match arg {
        "list" => {
            let mut desc = Vec::new();
            for i in name {
                desc.push(", ".to_owned());
                desc.push(format!("`?tag {}`", i));
            }
            embed = embed
                .title("List of Tag Commands")
                .description(desc[1..].concat());
        }
        _ => {
            if name.contains(&arg.to_owned()) {
                let item = tag
                    .tag
                    .iter()
                    .filter(|x| x.command == arg.to_owned())
                    .next()
                    .unwrap();
                embed = embed
                    .title(&item.command)
                    .description(&item.desc)
                    .image(&item.url);
            } else {
                let mut filtered = vec!["Maybe This Command is what you need\n".to_owned()];
                for i in name {
                    if dumb_matching(&i, arg) > 0.4 {
                        filtered.push(format!("`?tag {}`", i));
                        filtered.push(", ".to_owned());
                    }
                }
                let len = filtered.len() - 1;
                embed = embed
                    .title("Tag Not Found")
                    .description(filtered[..len].concat());
            }
        }
    };
    if let Some(x) = &bnd.msg.referenced_message {
        embed = embed.author(CreateEmbedAuthor::new(&x.author.name).icon_url(x.author.face()));
    }
    bnd.msg
        .channel_id
        .send_message(&bnd.ctx.http, Components::msg_reply_raw(bnd).embed(embed))
        .await?;
    Ok(())
}
