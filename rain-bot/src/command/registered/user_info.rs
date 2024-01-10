use crate::setup::AppData;
use crate::utils::MyColor;
use crate::{Context,MyErr};
use crate::model::MyContext;
use binding::postgres::card::Event;
use binding::utils::MyTime;
use poise::CreateReply;
use serenity::all::User;
use serenity::builder::{CreateEmbed, CreateEmbedAuthor};

pub fn currency(cur:i64)->String{
    let inp = cur.to_string(); 
    let x = inp.chars().rev();
    let mut y =Vec::new();
    for (i,c) in x.enumerate(){
        if i%3 == 0{
            y.push('.')
        }
        y.push(c)
    }
    let z:String = y[1..].iter().rev().collect();
    ["Bc ",&z,",00"].concat()
}

fn bbq_print(num:u8) -> String {
    let bbq = num+1;
    if bbq > 10 {
        return format!("BBQ{bbq}");
    }
    format!("BBQ0{bbq}")
}

fn decrypt_event(data:&str) -> Option<String> {
    let mut split = data.split("-");
    let categ = match split.next()?.parse::<u8>().ok()? {
        0 => "Bronze",
        1 => "Silver",
        2 => "Gold",
        3 => "Event",
        4 => "Free",
        5 => "Custom",
        _ => "idk"
    };
    let bbq = bbq_print(split.next()?.parse::<u8>().ok()?);
    Some(format!("{categ} {bbq}"))
}

fn event_embed(event:&Event,user:&User)->Result<CreateEmbed,MyErr>{
    let time = event.latest_bounty_time + 20*60*60;
    let time2 = event.latest_bounty_time + 40*60*60;
    let now = MyTime::now();
    let cd = |time: i64|{if time>=now{return format!("<t:{time}:R>");}"You can do it now".to_string()};
    let latest = decrypt_event(&event.latest_bounty).unwrap();
    let desc = format!("💰 Bounty Coin : {}\n🎫 Gacha Ticket : {} Ticket\n\n🕜 Latest Bounty : {latest}\n🕜 Time Completed : <t:{}:R>\n👨‍🌾 Different Bounty CD: {}\n👩‍🌾 Same Bounty CD: {}\n\n🥉 Bronze Stage : {}\n🥈 Silver Stage : {}\n🥇 Gold Stage: {}",currency(event.bounty as i64),event.gacha,event.latest_bounty_time,cd(time),cd(time2),bbq_print(event.bronze as u8),bbq_print(event.silver as u8),bbq_print(event.gold as u8));
    Ok(CreateEmbed::new()
        .author(CreateEmbedAuthor::new(&user.name).icon_url(user.face()))
        .title("Event Card")
        .description(desc)
        .color(MyColor::GREEN))
}

/// show other user card status info
/// use this to check your friend status
#[poise::command(context_menu_command = "User information",rename = "👤 Card")]
async fn card_context(
    ctx:Context<'_>,
    user:serenity::all::User
) -> Result<(),MyErr> {
    MyContext::from(ctx).card_command(Some(user)).await
}

/// show your own character data
#[poise::command(slash_command)]
async fn card(
    ctx:Context<'_>,
) -> Result<(),MyErr> {
    MyContext::from(ctx).card_command(None).await
}
/// show other user bounty status info
/// use this to check your friend status
#[poise::command(context_menu_command = "User information",rename="🎀 Event")]
async fn event_context(
    ctx:Context<'_>,
    user:serenity::all::User
) -> Result<(),MyErr> {
    MyContext::from(ctx).self_reg_model().await.complete_only()?;
    let event = ctx.data().db.get_event(&user.id.to_string()).await?;
    ctx.send(CreateReply::default().embed(event_embed(&event, &user)?)).await?;
    Ok(())
}

/// show your own bounty data
#[poise::command(slash_command)]
async fn event(
    ctx:Context<'_>,
) -> Result<(),MyErr> {
    if MyContext::from(ctx).registered_command().await?.is_some() {
        let event = ctx.data().db.get_event(&ctx.author().id.to_string()).await?;
        ctx.send(CreateReply::default().embed(event_embed(&event, ctx.author())?)).await?;
    }
    Ok(())
}


pub fn reg() -> Vec<poise::Command<AppData,MyErr>> {
    vec![event(),event_context(),card(),card_context()]
}
