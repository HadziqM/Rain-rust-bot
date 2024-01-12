use crate::reusable::utils;
use crate::{ComponentBundle, Components, Init, MyErr, Mybundle, Mytrait};
use serenity::all::*;

#[hertz::hertz_button_normal(0, false)]
async fn button(bnd: &ComponentBundle<'_>) -> Result<(), MyErr> {
    let mem = &bnd.cmd.member;
    let mut member = mem
        .clone()
        .ok_or(MyErr::Custom("youare not our server member".to_owned()))?;
    let role = RoleId::new(bnd.init.server_role.member_role);
    if !member.roles.contains(&role) {
        member.add_role(&bnd.ctx.http, role).await?;
    }
    Components::response(bnd, "you got the member role now", true).await?;
    Ok(())
}

pub(super) async fn join(ctx: &Context, member: &Member, init: &Init) -> Result<(), MyErr> {
    let arow = CreateActionRow::Buttons(vec![
        Components::normal_button("Join Server", "join", ButtonStyle::Primary, "🍏"),
        CreateButton::new_link(
            "https://discord.com/channels/937230168223789066/952959980749881434",
        )
        .label("Server Guide"),
    ]);
    let count = member
        .guild_id
        .to_partial_guild_with_counts(&ctx.http)
        .await?
        .approximate_member_count
        .unwrap();
    let embed = CreateEmbed::new().title("Welcome To Our Server")
        .author(CreateEmbedAuthor::new(&member.user.name).icon_url(member.face()))
        .image("https://media.discordapp.net/attachments/1068877927208452116/1069465096138526870/Rain_Wallpaper_Banner.png?width=1171&height=658")
        .description(format!("Welcome `{}` to our server,we are happy to get you as our {}'s members",&member.user.name,count))
        .color(utils::Color::Random.throw())
        .field("About Server", "As you might heard off, this server consist community of monster hunter frontier player across global,to make server better we already have set rule that we need to obey as player and as member of community", false)
        .field("About Rules", "You can press the button bellow to have access to our channel, we also send the rules with DM, hope you read it at leasure time", false)
        .field("About Channels", "To start with our server,we recommend to press the link button bellow to take you on our server channel mapping and information" , false);
    ChannelId::new(init.log_channel.join_channel)
        .send_message(
            &ctx.http,
            CreateMessage::new()
                .embed(embed)
                .components(vec![arow])
                .content(format!("{}", member.user.to_string())),
        )
        .await?;
    let rule1 = CreateEmbed::new().title("Greeting and Welcome to Rain Server 🎉 ").color(Color::TEAL).description("First of all, our goal is to create a healthy server that resembles the official server it used to be. we want players to really get the same experience and fun as the official server when playing and grinding to get certain items here. therefore we manage to have some rules to maintain balance and fairness in this server. Here are some basic rules that all member must read and follow:");
    let rule2 = CreateEmbed::new().title("In Game Rules").color(Color::MAGENTA).field("🛑 Cheating is bad", "> You are prohibited from using any kind of Cheats, especially `Cheat Engine` and other Memory Scanner/Debugger because it's very dangerous for the sustainability of our server. Any violation of this rule will be considered as an act of intentionally damaging the server and will be banned immediately (followed with other sanction)", false);
    let rule3 = CreateEmbed::new().title("Community Rules").color(Color::PURPLE).fields(
        vec![
        ("1️⃣  Mentions Everyone is bad","Please don't ever use the Everyone tag for mention the player, mention the user instead",false),
        ("2️⃣  Everyone Is Cool","Please be kind and respect each other, Positive Community is all we got. if you can't (or won't) give any advice for someone questions, just simply ignore it and don't make any scene. Showing off your big brain by putting other people down doesn't prove you are any better.",false),
        ("3️⃣  Server isnt a market place","You are prohibited to promote or advertise any content other than Monster Hunter themed without permission of Server's",false),
        ("4️⃣  No Horny","No **NSFW** content under any circumstances.",false),
        ("5️⃣  Dont Be Childish","No Fight, Flamming, Spam, and any other act that is against the rules that have been mentioned",false),
        ("6️⃣  No Cheating","Do not talk or even mention anything about cheats, any violation will get you Muted for a while, 3 times muted = auto kick",false),
        ]
        );
    member
        .user
        .direct_message(
            &ctx.http,
            CreateMessage::new().embeds(vec![rule1, rule2, rule3]),
        )
        .await?;
    Ok(())
}

pub(super) async fn leave(
    ctx: &Context,
    user: &User,
    guild: &GuildId,
    init: &Init,
) -> Result<(), MyErr> {
    let count = guild
        .to_partial_guild_with_counts(&ctx.http)
        .await?
        .approximate_member_count
        .unwrap();
    let roles = match &user.member {
        Some(x) => x
            .roles
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .concat(),
        None => String::from("cant get member info"),
    };
    let embed = CreateEmbed::new()
        .title("Member Leave")
        .description(format!(
            "{} leave at <t:{}:F>\n now we had {} member left",
            user.to_string(),
            utils::MyTime::now(),
            count
        ))
        .color(Color::DARK_RED)
        .field("Roles They Had", roles, false);
    ChannelId::new(init.log_channel.leave_channel)
        .send_message(&ctx.http, CreateMessage::new().embed(embed))
        .await?;
    Ok(())
}
