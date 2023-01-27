use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
    CommandDataOptionValue, ApplicationCommandInteraction,
};
use serenity::prelude::Context;
use crate::{PgConn,ErrorLog,Init};

pub async fn run(options: &[CommandDataOption],ctx:&Context, cmd:&ApplicationCommandInteraction,init:&Init) {
    let option = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");
    let mut error = ErrorLog::new(ctx, init,&cmd.user).await;
    if let CommandDataOptionValue::String(pass) = option {
        match PgConn::create(init, &cmd.user.id.to_string()).await {
            Ok(pg) => {
                match pg.change_user_password(pass.as_str()).await{
                    Ok(p)=>{
                        let out = match p {
                            true => "your password succesfully changed",
                            false => "you dont have any account in this server",
                        };
                        if let Err(why) = cmd.create_interaction_response(&ctx.http, |resp| {
                            resp.kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|msg|msg.content(out).ephemeral(true))
                        }).await{
                            error.change_error(why.to_string(),"change password", "discord connection problem,please consult");
                            error.log_error_channel().await;
                        }
                    }
                    Err(why)=>{
                        error.change_error(why.to_string(), "changing account", "this one rare, please consult");
                        error.log_slash(cmd, false).await;
                    }
                }
            }
            Err(why) => {
                error.change_error(why.to_string(),"change password", "wait for server to be more stable")
            }
        }
    }
}
