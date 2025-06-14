use std::fmt::Display;

use common::setting::SettingAll;
use database::DbError;
use serenity::all::{
    ChannelId, Color, Context, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, User, UserId,
};
use thiserror::Error;

pub type MyResult<T> = Result<T, MyError>;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("{0}")]
    Custom(String),
    #[error("sqlx error: {0}")]
    Db(DbError),
    #[error("discord API error: {0}")]
    Serenity(#[from] serenity::Error),
    #[error("Tokio IO error: {0}")]
    Tokio(#[from] tokio::io::Error),
}

impl From<&str> for MyError {
    fn from(err: &str) -> Self {
        MyError::Custom(err.to_string())
    }
}

impl From<DbError> for MyError {
    fn from(err: DbError) -> Self {
        match err {
            DbError::Sqlx(_) => Self::Db(err),
            DbError::Custom(str) => Self::Custom(str),
        }
    }
}

pub enum Severity {
    Critical,
    FalsePositive,
    CanBeHandledManually,
}

#[derive(Debug, Clone, Copy)]
pub enum CommandLocationType {
    Slash,
    Button,
    Modal,
    Message,
    Other,
}

impl MyError {
    pub fn severity(&self) -> Severity {
        match self {
            Self::Custom(_) => Severity::CanBeHandledManually,
            Self::Db(_) => Severity::Critical,
            MyError::Serenity(_) => Severity::FalsePositive,
            Self::Tokio(_) => Severity::Critical,
        }
    }
    pub fn advice(&self) -> String {
        match self {
            Self::Custom(_) => {
                String::from("Error written by author themself, please read carefully")
            }
            Self::Db(_) => String::from(
                "Please report this error to author, or wait till database connection stabilize",
            ),
            Self::Serenity(_) => String::from("Discord API error, please report this error"),
            Self::Tokio(_) => String::from("Tokio IO error, please report this error"),
        }
    }
    pub fn log(&self, location: impl Display, user: impl Display, ctype: CommandLocationType) {
        match self {
            Self::Custom(err) => {
                log::warn!(
                    "\n[Custom Error]\ndetails: `{err}`\non_command: `{location}`\ncommand_type: `{ctype:?}`\nuser: `{user}`"
                )
            }
            Self::Db(err) => log::error!("\n[Database Error]\ndetails: `{err}`\non_command: `{location}`\ncommand_type: `{ctype:?}`\nuser: `{user}`"),
            Self::Serenity(err) => {
                log::warn!("\n[Serenity Error]\ndetails: `{err}`\non_command: `{location}`\ncommand_type: `{ctype:?}`\nuser: `{user}`")
            }
            Self::Tokio(err) => {
                log::warn!("\n[Tokio Error]\ndetails: `{err}`\non_command: `{location}`\ncommand_type: `{ctype:?}`\nuser: `{user}`")
            }
        }
    }
}

pub struct ErrorHandling {
    pub err: MyError,
    pub author: User,
    pub effected_user: User,
    pub location: String,
    pub log_channel: u64,
    pub ctype: CommandLocationType,
}

impl ErrorHandling {
    pub async fn new(
        err: MyError,
        ctx: &Context,
        setting: &SettingAll,
        effected_user: User,
        location: String,
        ctype: CommandLocationType,
    ) -> Self {
        let thor = UserId::new(setting.main.discord.author);
        let author = thor.to_user(&ctx.http).await.unwrap_or_default();
        Self {
            err,
            author,
            effected_user,
            location,
            ctype,
            log_channel: setting.discord.channel.error_channel,
        }
    }

    pub fn response(&self) -> CreateInteractionResponse {
        self.err
            .log(&self.location, &self.effected_user.name, self.ctype);
        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().embed(self.embed()),
        )
    }
    pub async fn channel_send(&self, ctx: &Context) {
        let ch = ChannelId::new(self.log_channel);
        if ch
            .send_message(
                &ctx.http,
                CreateMessage::new()
                    .embed(self.embed())
                    .content(format!("for {}", self.effected_user)),
            )
            .await
            .is_err()
        {
            log::error!(
                "error sending log to channel, the error is: {:?}, on: {}, and user effected: {}",
                self.err,
                self.location,
                self.effected_user.name
            )
        }
    }

    pub fn embed(&self) -> CreateEmbed {
        let color = match self.err.severity() {
            Severity::Critical => Color::RED,
            Severity::FalsePositive => Color::DARK_GREEN,
            Severity::CanBeHandledManually => Color::ORANGE,
        };
        CreateEmbed::default()
            .color(color)
            .title("🛑 Error Occurred 🛑")
            .description("some cant be handled error Occurred")
            .fields(vec![
                (
                    "🚧 Occurred on",
                    format!("**{}**", self.location.to_uppercase()),
                    false,
                ),
                ("📜 error message", format!("> {}", &self.err), false),
                (
                    "⛑  author advice",
                    format!("```\n{}\n```", &self.err.advice()),
                    false,
                ),
            ])
            .author(
                CreateEmbedAuthor::new(&self.effected_user.name)
                    .icon_url(self.effected_user.face()),
            )
            .footer(
                CreateEmbedFooter::new(format!("you can consult this to {}", self.author.tag()))
                    .icon_url(self.author.face()),
            )
            .thumbnail("https://media.discordapp.net/attachments/1068877712841789490/1303771799246344272/panics.png?ex=6730435b&is=672ef1db&hm=5f6e31b20eb02607c29ae1a961d97b694dee45ccbdc7889638edc4a6f93903a3&=&format=webp&quality=lossless&width=1000&height=660")
    }
}
