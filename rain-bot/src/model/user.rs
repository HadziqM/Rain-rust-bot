use super::components::Components;
use super::MyContext;
use crate::utils::MyColor;
use crate::Context;
use crate::{error::MyErr, setup::AppData};
use binding::postgres::card::Card;
use poise::CreateReply;
use serenity::all::User;
use serenity::all::{ButtonStyle, CreateActionRow};
use serenity::builder::{CreateEmbed, CreateEmbedFooter};
use serenity::futures::StreamExt;
use tokio::time::Duration;

#[derive(Debug)]
pub enum UserReg {
    Complete { uid: i32, uname: String, cid: i32 },
    NoCharacter { uid: i32, uname: String, err: MyErr },
    Unregistered { err: MyErr },
}

impl MyContext<'_> {
    pub async fn self_reg_model(&self) -> UserReg {
        self.data().user_reg(&self.author().id.to_string()).await
    }
    pub async fn card_command(&self, user: Option<User>) -> Result<(), MyErr> {
        let cid;
        let usr;
        if let Some(us) = user {
            cid = self
                .data()
                .user_reg(&us.id.to_string())
                .await
                .complete_only()?
                .2;
            usr = us;
        } else {
            match self.registered_command().await? {
                Some((x, _, _)) => cid = x,
                None => {
                    return Ok(());
                }
            }
            usr = self.author().clone();
        }
        let card = self.data().db.get_card(cid).await?;
        self.send(CreateReply::default().embed(card_embed(&card, &usr)))
            .await?;
        Ok(())
    }
    pub async fn registered_command(&self) -> Result<Option<(i32, String, i32)>, MyErr> {
        let (uid, uname, ucid) = self
            .data()
            .user_reg(&self.author().id.to_string())
            .await
            .non_unreg()?;
        match ucid {
            Some(x) => Ok(Some((uid, uname, x))),
            None => {
                card_select(**self, self.author(), uid).await?;
                return Ok(None);
            }
        }
    }
    pub async fn switch_command(&self) -> Result<(), MyErr> {
        let (uid, _, _) = self.self_reg_model().await.non_unreg()?;
        card_select(**self, self.author(), uid).await
    }
}

impl AppData {
    pub async fn user_reg(&self, did: &str) -> UserReg {
        match self.db.get_user(did).await {
            Ok(data) => match self.db.get_char_id(did).await {
                Ok(cid) => UserReg::Complete {
                    uid: data.0,
                    uname: data.1,
                    cid,
                },
                Err(err) => UserReg::NoCharacter {
                    uid: data.0,
                    uname: data.1,
                    err: err.into(),
                },
            },
            Err(err) => UserReg::Unregistered { err: err.into() },
        }
    }
}

impl UserReg {
    pub fn unreg_only(&self) -> Result<(), MyErr> {
        if let Self::Unregistered { err: _ } = self {
            return Ok(());
        }
        Err(MyErr::from(
            "You already registered, this command are only for unregistered",
        ))
    }
    pub fn complete_only(self) -> Result<(i32, String, i32), MyErr> {
        match self {
            Self::Complete { uid, uname, cid } => {
                Ok((uid.to_owned(), uname.to_owned(), cid.to_owned()))
            }
            Self::Unregistered { err } => Err(err),
            Self::NoCharacter {
                uid: _,
                uname: _,
                err,
            } => Err(err),
        }
    }
    pub fn non_unreg(self) -> Result<(i32, String, Option<i32>), MyErr> {
        match self {
            Self::Complete { uid, uname, cid } => {
                Ok((uid.to_owned(), uname.to_owned(), Some(cid.to_owned())))
            }
            Self::Unregistered { err } => Err(err),
            Self::NoCharacter { uid, uname, err: _ } => {
                Ok((uid.to_owned(), uname.to_owned(), None))
            }
        }
    }
}

fn make_button(len: usize) -> CreateActionRow {
    let mut button = vec![Components::normal_button(
        "use",
        "use",
        ButtonStyle::Primary,
        "👍",
    )];
    if len != 1 {
        button.push(Components::normal_button(
            "next",
            "next",
            ButtonStyle::Success,
            "➡️",
        ));
    }
    CreateActionRow::Buttons(button)
}

fn card_embed(card: &Card, user: &User) -> CreateEmbed {
    CreateEmbed::new()
        .title(card.name.as_str())
        .fields(vec![
            (
                "User",
                &format!(
                    "username: {}\nuser_id: {}\nchar_id: {}\nlast_login: {}",
                    &card.username,
                    card.user_id,
                    card.char_id,
                    card.last_login()
                ),
                false,
            ),
            (
                "Character",
                &format!("HR: {}\nGR: {}", card.hrp(), card.gr),
                false,
            ),
            (
                "Guild",
                &format!("name: {}\nguild_id: {}", &card.g_name(), &card.g_id()),
                false,
            ),
        ])
        .footer(CreateEmbedFooter::new(format!("owned by {}", user.name)).icon_url(user.face()))
        .colour(MyColor::random())
        .thumbnail(&card.get_path())
}

fn reply_card(card: &Vec<Card>, user: &User, index: usize) -> CreateReply {
    CreateReply::default()
        .embed(card_embed(&card[index], user))
        .components(vec![make_button(card.len())])
        .ephemeral(true)
}

async fn card_select(ctx: Context<'_>, user: &User, uid: i32) -> Result<(), MyErr> {
    let card = ctx.data().db.get_all_card(uid).await?;
    if card.len() == 0 {
        return Err(MyErr::Custom("you doesnt have any charachter on your account, please make one on the game launcher and use it to enter town first before using this command again".to_string()));
    }
    let mut index = 0;
    let send = ctx.send(reply_card(&card, user, index)).await?;
    let msg = send.message().await?;
    let mut reply = msg
        .await_component_interactions(ctx)
        .timeout(Duration::from_secs(60 * 3))
        .stream();
    while let Some(pat) = reply.next().await {
        let id = &pat.data.custom_id;
        if user != &pat.user {
            continue;
        }
        if id == "next" {
            index += 1;
            if index == card.len() {
                index = 0;
            }
            send.edit(ctx, reply_card(&card, user, index)).await?;
            pat.defer(ctx).await?;
        } else if id == "use" {
            ctx.data()
                .db
                .switch(card[index].char_id, &user.id.to_string())
                .await?;
            pat.create_response(
                ctx,
                Components::interaction_response("successfully switch main character", true),
            )
            .await?;
            break;
        }
    }
    send.delete(ctx).await?;
    Ok(())
}
