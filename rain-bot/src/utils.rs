#![allow(async_fn_in_trait)]

pub mod all {
    pub use crate::error::{MyError, MyResult};
    pub use crate::setup::*;
    pub use crate::MyApp as App;
    pub use serenity::all::*;
}

pub use all::*;

pub struct AppReg;

impl AppReg {
    pub fn user_context(name: impl Into<String>) -> CreateCommand {
        CreateCommand::new(name).kind(CommandType::User)
    }
    pub fn message_context(name: impl Into<String>) -> CreateCommand {
        CreateCommand::new(name).kind(CommandType::Message)
    }
    pub fn normal_slash(name: impl Into<String>, desc: impl Into<String>) -> CreateCommand {
        CreateCommand::new(name).description(desc)
    }
    pub fn admin_slash(name: impl Into<String>, desc: impl Into<String>) -> CreateCommand {
        CreateCommand::new(name)
            .description(desc)
            .default_member_permissions(Permissions::ADMINISTRATOR)
    }
    pub fn subcommand(name: impl Into<String>, desc: impl Into<String>) -> CreateCommandOption {
        CreateCommandOption::new(serenity::all::CommandOptionType::SubCommand, name, desc)
    }
    pub fn user_option(name: impl Into<String>, desc: impl Into<String>) -> CreateCommandOption {
        CreateCommandOption::new(serenity::all::CommandOptionType::User, name, desc)
    }
    pub fn int_option(name: impl Into<String>, desc: impl Into<String>) -> CreateCommandOption {
        CreateCommandOption::new(serenity::all::CommandOptionType::Integer, name, desc)
    }
    pub fn att_option(name: impl Into<String>, desc: impl Into<String>) -> CreateCommandOption {
        CreateCommandOption::new(serenity::all::CommandOptionType::Attachment, name, desc)
    }
    pub fn str_option(name: impl Into<String>, desc: impl Into<String>) -> CreateCommandOption {
        CreateCommandOption::new(serenity::all::CommandOptionType::String, name, desc)
    }
}

pub struct Components;

impl Components {
    pub fn normal_button(
        name: &str,
        custom_id: &str,
        style: ButtonStyle,
        emoji: &str,
    ) -> CreateButton {
        let mut b = CreateButton::new(custom_id).label(name).style(style);
        if let Ok(emj) = emoji.parse::<ReactionType>() {
            b = b.emoji(emj);
        }
        b
    }
    pub fn interaction_response(content: &str, ephemeral: bool) -> CreateInteractionResponse {
        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content(content)
                .ephemeral(ephemeral),
        )
    }
    pub async fn response<T: JoinCommandTrait>(
        cmd: &T,
        ctx: &Context,
        content: &str,
        ephemeral: bool,
    ) -> MyResult<()> {
        cmd.response(ctx, Components::interaction_response(content, ephemeral))
            .await
    }
    pub async fn edit<T: JoinCommandTrait>(cmd: &T, ctx: &Context, content: &str) -> MyResult<()> {
        let rply = EditInteractionResponse::new().content(content);
        cmd.edit(ctx, rply).await
    }
    pub async fn msg(msg: Message, ctx: &Context, content: &str) -> MyResult<Message> {
        if content.len() >= 2000 {
            return Err(MyError::Custom(
                "the result is higher than 2000 char,and discord doesnt allow it".to_string(),
            ));
        }
        Ok(msg
            .channel_id
            .send_message(&ctx.http, CreateMessage::new().content(content))
            .await?)
    }
    pub fn sub_options(cmd: &CommandInteraction) -> MyResult<&Vec<CommandDataOption>> {
        for data in &cmd.data.options {
            if let CommandDataOptionValue::SubCommand(x) = &data.value {
                return Ok(x);
            }
        }
        Err(MyError::Custom("cant find subcommand".to_string()))
    }
    pub fn get_mentions(mention: &str) -> Vec<UserId> {
        let mut out = Vec::new();
        for i in mention.split(">") {
            let val = if i.contains("<!@") {
                i.replace("<!@", "").trim().to_owned()
            } else {
                i.replace("<@", "").trim().to_owned()
            };

            if let Ok(id) = val.parse::<u64>() {
                out.push(UserId::new(id));
            }
        }
        out
    }
    pub async fn add_role(member: Member, ctx: &Context, role: u64) -> MyResult<()> {
        let role = RoleId::new(role);
        if !member.roles.contains(&role) {
            let _ = member.add_role(&ctx.http, role).await;
        }
        Ok(())
    }
    pub async fn remove_role(member: Member, ctx: &Context, role: u64) -> MyResult<()> {
        let role = RoleId::new(role);
        if member.roles.contains(&role) {
            let _ = member.remove_role(&ctx.http, role).await;
        }
        Ok(())
    }
}

pub trait JoinCommandTrait: Sized + 'static {
    async fn response(&self, ctx: &Context, reply: CreateInteractionResponse) -> MyResult<()>;
    async fn defer_response(&self, ctx: &Context, ephemeral: bool) -> MyResult<()>;
    async fn get_message(&self, ctx: &Context) -> MyResult<Message>;
    async fn edit(&self, ctx: &Context, reply: EditInteractionResponse) -> MyResult<()>;
    fn user(&self) -> User;
    fn discord_id(&self) -> String {
        self.user().id.to_string()
    }
}

impl JoinCommandTrait for CommandInteraction {
    async fn response(&self, ctx: &Context, reply: CreateInteractionResponse) -> MyResult<()> {
        self.create_response(&ctx.http, reply).await?;
        Ok(())
    }
    async fn defer_response(&self, ctx: &Context, ephemeral: bool) -> MyResult<()> {
        match ephemeral {
            true => self.defer_ephemeral(&ctx.http).await?,
            false => self.defer(&ctx.http).await?,
        }
        Ok(())
    }
    async fn get_message(&self, ctx: &Context) -> MyResult<Message> {
        Ok(self.get_response(&ctx.http).await?)
    }
    async fn edit(&self, ctx: &Context, reply: EditInteractionResponse) -> MyResult<()> {
        self.edit_response(&ctx.http, reply).await?;
        Ok(())
    }

    fn user(&self) -> User {
        self.user.clone()
    }
}

impl JoinCommandTrait for ComponentInteraction {
    fn user(&self) -> User {
        self.user.clone()
    }

    async fn response(&self, ctx: &Context, reply: CreateInteractionResponse) -> MyResult<()> {
        self.create_response(&ctx.http, reply).await?;
        Ok(())
    }

    async fn defer_response(&self, ctx: &Context, ephemeral: bool) -> MyResult<()> {
        match ephemeral {
            true => self.defer_ephemeral(&ctx.http).await?,
            false => self.defer(&ctx.http).await?,
        }
        Ok(())
    }
    async fn edit(&self, ctx: &Context, reply: EditInteractionResponse) -> MyResult<()> {
        self.edit_response(&ctx.http, reply).await?;
        Ok(())
    }

    async fn get_message(&self, ctx: &Context) -> MyResult<Message> {
        Ok(self.get_response(&ctx.http).await?)
    }
}

impl JoinCommandTrait for ModalInteraction {
    fn user(&self) -> User {
        self.user.clone()
    }
    async fn edit(&self, ctx: &Context, reply: EditInteractionResponse) -> MyResult<()> {
        self.edit_response(&ctx.http, reply).await?;
        Ok(())
    }

    async fn response(&self, ctx: &Context, reply: CreateInteractionResponse) -> MyResult<()> {
        self.create_response(&ctx.http, reply).await?;
        Ok(())
    }

    async fn defer_response(&self, ctx: &Context, ephemeral: bool) -> MyResult<()> {
        match ephemeral {
            true => self.defer_ephemeral(&ctx.http).await?,
            false => self.defer(&ctx.http).await?,
        }
        Ok(())
    }

    async fn get_message(&self, ctx: &Context) -> MyResult<Message> {
        Ok(self.get_response(&ctx.http).await?)
    }
}
