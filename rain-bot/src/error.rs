use crate::utils::MyColor;
use crate::{AppData, Context};
use binding::postgres::PgCustomError;
use serenity::all::ChannelId;
use serenity::builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage};

#[derive(Debug)]
pub enum MyErr {
    Serenity(serenity::Error),
    Tokio(tokio::io::Error),
    Utf8(std::str::Utf8Error),
    Serde(serde_json::Error),
    ByteWise(binding::bitwise::BitwiseError),
    Image(image_edit::CustomImageError),
    Custom(String),
}
impl std::error::Error for MyErr {}
impl std::fmt::Display for MyErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyErr::Tokio(x) => x.fmt(f),
            MyErr::Serenity(x) => x.fmt(f),
            MyErr::Utf8(x) => x.fmt(f),
            MyErr::Serde(x) => x.fmt(f),
            MyErr::Custom(x) => x.fmt(f),
            MyErr::ByteWise(x) => x.fmt(f),
            MyErr::Image(x) => x.fmt(f),
        }
    }
}
impl From<binding::bitwise::BitwiseError> for MyErr {
    fn from(value: binding::bitwise::BitwiseError) -> Self {
        MyErr::ByteWise(value)
    }
}
impl From<tokio::io::Error> for MyErr {
    fn from(value: tokio::io::Error) -> Self {
        MyErr::Tokio(value)
    }
}
impl From<serenity::Error> for MyErr {
    fn from(value: serenity::Error) -> Self {
        MyErr::Serenity(value)
    }
}
impl From<std::str::Utf8Error> for MyErr {
    fn from(value: std::str::Utf8Error) -> Self {
        MyErr::Utf8(value)
    }
}
impl From<serde_json::Error> for MyErr {
    fn from(value: serde_json::Error) -> Self {
        MyErr::Serde(value)
    }
}
impl From<image_edit::CustomImageError> for MyErr {
    fn from(value: image_edit::CustomImageError) -> Self {
        MyErr::Image(value)
    }
}
impl From<&str> for MyErr {
    fn from(value: &str) -> Self {
        MyErr::Custom(value.to_string())
    }
}
impl From<binding::postgres::PgCustomError> for MyErr {
    fn from(value: binding::postgres::PgCustomError) -> Self {
        match value {
            PgCustomError::Sqlx(x) => MyErr::ByteWise(x.into()),
            PgCustomError::Custom(x) => MyErr::Custom(x),
        }
    }
}
impl From<binding::bounty::BountyErr> for MyErr {
    fn from(value: binding::bounty::BountyErr) -> Self {
        match value {
            binding::bounty::BountyErr::Tokio(x) => MyErr::Tokio(x),
            binding::bounty::BountyErr::Serde(x) => MyErr::Serde(x),
            binding::bounty::BountyErr::Custom(x) => MyErr::Custom(x),
        }
    }
}

impl MyErr {
    fn severity(&self) -> bool {
        match self {
            MyErr::Custom(_) => false,
            MyErr::ByteWise(_) => true,
            MyErr::Utf8(_) => false,
            MyErr::Tokio(_) => true,
            MyErr::Serde(_) => false,
            MyErr::Serenity(_) => false,
            MyErr::Image(_) => true,
        }
    }
    fn advice(&self) -> String {
        match self {
            MyErr::Custom(_)=>"Error message is writen by author themself, please read the message carefully or consult".to_string(),
            MyErr::ByteWise(_)=>"postgres connection (server database) error or data format error, you can report this or try again".to_string(),
            MyErr::Utf8(_)=>"parsing error while analizing file, are you sure you send a safe file?".to_string(),
            MyErr::Tokio(_)=>"file system error or paralel thread broken, report this!".to_string(),
            MyErr::Serde(_)=>"failed to operate with json data, that file might be broken or wrong format, carafully read the error message, it will tell which line in the file is wrong/invalid, then edit those file again".to_string(),
            MyErr::Serenity(_)=>"discord error, well discord unreasonably do this sometime, but rest assured, whatever you do, its already finished successfully, but if you find you missing something, you could report this".to_string(),
            MyErr::Image(_)=>"error on loading image or at image processing, you can report this to be investigated".to_owned()
        }
    }
    fn embed(&self, ctx: Context<'_>) -> CreateEmbed {
        let user = ctx.author();
        let color = self
            .severity()
            .then_some(MyColor::RED)
            .unwrap_or(MyColor::YELLOW);
        CreateEmbed::new()
            .title("🛑 Error Occured 🛑")
            .description("some cant be handled error occured")
            .fields(vec![
                ("🚧 occured on",format!("**{}**",ctx.invoked_command_name().to_uppercase()),false),
                ("📜 error message",format!("> {}",self.to_string()),false),
                ("⛑  author advice",format!("```\n{}\n```",self.advice()),false)
            ])
            .author(CreateEmbedAuthor::new(&user.name).icon_url(&user.face()))
            .footer(CreateEmbedFooter::new(format!("you can consult this to {}",user.name))
                .icon_url(&user.face()))
            .color(color)
            .thumbnail("https://media.discordapp.net/attachments/1009291538733482055/1185000150104543314/panics.png?ex=658e0464&is=657b8f64&hm=28866be3c84841d0391a59ce797257cd661c5dff6e72d3f01b0f4f11ba4eac10&=&format=webp&quality=lossless&width=709&height=468")
    }
    fn reply(&self, ctx: Context<'_>) -> poise::CreateReply {
        poise::CreateReply {
            embeds: vec![self.embed(ctx)],
            ..Default::default()
        }
    }
    async fn log_channel(&self, ctx: Context<'_>) -> Result<(), MyErr> {
        let ch_id = ChannelId::new(ctx.data().init.read().await.log_channel.err_channel);
        ch_id
            .send_message(
                ctx.serenity_context(),
                CreateMessage::new()
                    .content(format!("for {}", ctx.author().to_string()))
                    .embed(self.embed(ctx)),
            )
            .await?;
        Ok(())
    }
    pub async fn on_error(err: poise::FrameworkError<'_, AppData, Self>) {
        match err {
            poise::FrameworkError::Setup { error, .. } => {
                panic!("failed to setup with err messages {error:?}")
            }
            poise::FrameworkError::Command { error, ctx, .. } => {
                if ctx.send(error.reply(ctx)).await.is_err() {
                    let _ = error.log_channel(ctx).await;
                }
            }
            error => {
                if let Err(e) = poise::builtins::on_error(error).await {
                    println!("Error while handling error: {}", e)
                }
            }
        }
    }
}
