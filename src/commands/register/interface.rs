use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::component::ButtonStyle;
use serenity::model::prelude::interaction::application_command::{CommandDataOption, ApplicationCommandInteraction};
use serenity::prelude::Context;
use serenity::model::prelude::interaction::InteractionResponseType;
use crate::reusable::component::button::normal_button;
use crate::{Init,ErrorLog};
use crate::reusable::utils::color;

pub async fn run(_options: &[CommandDataOption],ctx:&Context,cmd:&ApplicationCommandInteraction,init:&Init){
    if let Err(why) = cmd.create_interaction_response(&ctx.http, |resp| {
        resp.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|msg|{
                msg.embed(|emb|{
                    emb.title("MHFZ user interface")
                        .color(color("40", "ff", "40"))
                        .description("button interface for mhfz player to make use of server's utility")
                }).components(|c|{
                        c.create_action_row(|r|{
                            r.add_button(normal_button("register", "register_i", ButtonStyle::Primary,"📝".parse().unwrap()))
                            .add_button(normal_button("bind", "bind_i", ButtonStyle::Secondary,"🔐".parse().unwrap()))
                        })
                        .create_action_row(|r|{
                            r.add_button(normal_button("transfer", "transfer_i", ButtonStyle::Primary,"⏳".parse().unwrap()))
                            .add_button(normal_button("DM save", "dm_save_i", ButtonStyle::Secondary,"🎁".parse().unwrap()))
                        })
                    })
            })
    }).await{
        let mut err = ErrorLog::new(&ctx, init, &cmd.user).await;
        err.change_error(why.to_string(), "error command", "it just test woles");
        err.log_error_channel().await;
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("interface").description("An User interface for MHFZ player")
}
