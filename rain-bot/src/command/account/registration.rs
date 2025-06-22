use common::setting::SettingAll;

use crate::all::*;

struct RegisteredMessage {
    name: String,
    uid: i32,
    register: bool,
    member: Member,
}

impl RegisteredMessage {
    fn new(name: String, uid: i32, register: bool, member: Member) -> Self {
        RegisteredMessage {
            name,
            uid,
            register,
            member,
        }
    }

    async fn add_role(&self, ctx: &Context, setting: &SettingAll) -> MyResult<()> {
        let role = RoleId::new(setting.discord.role.registered);
        self.member.add_role(&ctx.http, role).await?;
        Ok(())
    }

    fn embed(&self) -> CreateInteractionResponse {
        let word = || {
            if self.register {
                return "Created";
            }
            "binded"
        };
        let embed = CreateEmbed::new().title(format!("Account Successfully {} on Server",word()))
                .description(format!("{} {} an account on server, remember that you still need to have a character in game to fully use our discord features so hurry up and create one if you havent",self.member,word())).fields(vec![
                    ("👤 Username",&format!("`{}`",self.name),false),
                    ("🆔 User Id",&format!("`{}`",self.uid),false)
                ]).author(CreateEmbedAuthor::new(self.member.display_name()).icon_url(self.member.face()))
            .colour(Colour::DARK_GREEN)
            .image("https://media.discordapp.net/attachments/1068440173479739393/1068458599627620392/cachedImage.png?width=807&height=455");

        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().embeds(vec![embed]),
        )
    }
}

fn modal_response(reg: bool) -> CreateInteractionResponse {
    let name;
    let title;
    if reg {
        name = "register";
        title = "Register Command";
    } else {
        name = "bind";
        title = "Bind Command";
    }
    CreateInteractionResponse::Modal(CreateModal::new(name, title).components(vec![
            modal_register_row("username", false),
            modal_register_row("password", true),
            CreateActionRow::InputText(
                CreateInputText::new(InputTextStyle::Short, "PSN_ID", "psn_id")
                    .required(false)
                    .placeholder("For console player (You can Leave it Empty)"),
            ),
        ]))
}

fn modal_register_row(name: &str, pass: bool) -> CreateActionRow {
    let placeholder = match pass {
        false => "your MHFZ username on launcher".to_owned(),
        true => "your MHFZ user password (ignore discord warning)".to_owned(),
    };
    CreateActionRow::InputText(
        CreateInputText::new(InputTextStyle::Short, name, name)
            .required(true)
            .placeholder(&placeholder),
    )
}

async fn handle_modal_fn(
    app: Arc<App>,
    cmd: ModalInteraction,
    ctx: Context,
    register: bool,
) -> MyResult<()> {
    app.only_unregister_user(&cmd.user).await?;
    let mut name = String::new();
    let mut password = String::new();
    let mut psn = None;
    for comp in &cmd.data.components {
        let arow = comp.components.first().unwrap();
        if let ActionRowComponent::InputText(input) = arow {
            match input.custom_id.as_str() {
                "username" => name = input.value.clone().unwrap(),
                "password" => password = input.value.clone().unwrap(),
                _ => psn = input.value.to_owned(),
            }
        }
    }

    let data = app
        .db
        .add_account(&name, &password, &cmd.discord_id(), !register, psn)
        .await?;

    if let Some(member) = cmd.member.clone() {
        let reg = RegisteredMessage::new(name.clone(), data.id, register, member);
        let setting = app.setting.read().await;
        reg.add_role(&ctx, &setting).await?;
        cmd.create_response(&ctx.http, reg.embed()).await?;
    }

    Ok(())
}

pub struct RegisterAccount;
pub struct BindAccount;

impl RegisterAccount {
    pub fn name(&self) -> &'static str {
        "register"
    }
}
impl BindAccount {
    pub fn name(&self) -> &'static str {
        "bind"
    }
}

/// register = true, bind = false
pub async fn registration(cmd: CommandInteraction, ctx: Context, regis: bool) -> MyResult<()> {
    cmd.create_response(&ctx.http, modal_response(regis))
        .await?;
    Ok(())
}

#[async_trait]
impl ButtonInteractionTrait for RegisterAccount {
    fn name_btn(&self) -> &'static str {
        self.name()
    }
    async fn handle_button(
        &self,
        _app: Arc<App>,
        cmd: ComponentInteraction,
        ctx: Context,
    ) -> MyResult<()> {
        cmd.create_response(&ctx.http, modal_response(true)).await?;
        Ok(())
    }
}
#[async_trait]
impl ButtonInteractionTrait for BindAccount {
    fn name_btn(&self) -> &'static str {
        self.name()
    }
    async fn handle_button(
        &self,
        _app: Arc<App>,
        cmd: ComponentInteraction,
        ctx: Context,
    ) -> MyResult<()> {
        cmd.create_response(&ctx.http, modal_response(false))
            .await?;
        Ok(())
    }
}

#[async_trait]
impl ModalInteractionTrait for RegisterAccount {
    fn name_mdl(&self) -> &'static str {
        self.name()
    }
    async fn handle_modal(
        &self,
        app: Arc<App>,
        cmd: ModalInteraction,
        ctx: Context,
    ) -> MyResult<()> {
        handle_modal_fn(app, cmd, ctx, true).await
    }
}
#[async_trait]
impl ModalInteractionTrait for BindAccount {
    fn name_mdl(&self) -> &'static str {
        self.name()
    }
    async fn handle_modal(
        &self,
        app: Arc<App>,
        cmd: ModalInteraction,
        ctx: Context,
    ) -> MyResult<()> {
        handle_modal_fn(app, cmd, ctx, false).await
    }
}
