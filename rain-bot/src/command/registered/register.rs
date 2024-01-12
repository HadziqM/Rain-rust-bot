use crate::{model::MyContext, setup::AppData, utils::MyColor, ApplicationContext, Context, MyErr};
use binding::postgres::account::AccountData;
use poise::{CreateReply, Modal};
use serenity::{
    all::{ChannelId, RoleId},
    builder::{CreateEmbed, CreateMessage},
};

#[derive(Debug, Modal)]
#[name = "Register/Bind Form"]
struct RegMod {
    #[placeholder = "your username in game (case sensitive)"]
    username: String,
    #[placeholder = "ignore discord warn"]
    password: String,
    #[placeholder = "fill only if you plan to play on console"]
    psn_id: Option<String>,
}

fn reg_embed(reg: bool, user: AccountData, app: MyContext<'_>) -> CreateEmbed {
    let name = reg.then_some("Registered").unwrap_or("Binded");
    let desc = reg.then_some("Use username and password to login into the game, then create character on launcher, enjoy the game").unwrap_or("you can now fully use our discord features");
    CreateEmbed::new()
        .title(format!("Account Succesfully {} on Server", name))
        .description(desc)
        .fields(vec![
            ("👤 Username", &format!("`{}`", &user.username), false),
            ("🆔 User Id", &format!("`{}`", user.id), false),
        ])
        .color(MyColor::GREEN)
        .author(app.embed_author())
}

pub async fn modal_invoke(ctx: ApplicationContext<'_>, reg: bool) -> Result<(), MyErr> {
    let app = MyContext::from(Context::from(ctx));
    if let Some(data) = RegMod::execute(ctx).await? {
        let user = ctx
            .data()
            .db
            .create_account(
                &data.username,
                &data.password,
                data.psn_id.clone(),
                reg,
                &ctx.author().id.to_string(),
            )
            .await?;
        // select character if bind
        if !reg {
            ctx.say("account are partially binded, proceed to next step")
                .await?;
            app.switch_command().await?;
        }
        // add role
        let role = RoleId::new(ctx.data.init.read().await.server_role.register_role);
        app.self_assign_role(role).await?;

        // notify user and log
        let reply = reg_embed(reg, user, app);
        ctx.send(CreateReply::default().embed(reply.clone()).ephemeral(true))
            .await?;

        let channel = ChannelId::new(ctx.data.init.read().await.log_channel.account_channel);
        channel
            .send_message(ctx, CreateMessage::new().embed(reply))
            .await?;
    }
    Ok(())
}

/// register game account from discord
#[poise::command(slash_command)]
async fn register(ctx: ApplicationContext<'_>) -> Result<(), MyErr> {
    modal_invoke(ctx, true).await
}
/// bind game account into discord
#[poise::command(slash_command)]
async fn bind(ctx: ApplicationContext<'_>) -> Result<(), MyErr> {
    modal_invoke(ctx, false).await
}

/// add psn id if havent
#[poise::command(slash_command)]
async fn add_psn(
    ctx: Context<'_>,
    #[description = "your console psn id"] psn_id: String,
) -> Result<(), MyErr> {
    if let Some((uid, _, _)) = MyContext::from(ctx).registered_command().await? {
        ctx.data().db.add_psn(&psn_id, uid).await?;
        ctx.say("psn id already binded").await?;
    }
    Ok(())
}

pub fn reg() -> Vec<poise::Command<AppData, MyErr>> {
    vec![register(), bind(), add_psn()]
}
