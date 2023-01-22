use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::component::ButtonStyle;
use serenity::model::prelude::interaction::application_command::{CommandDataOption, ApplicationCommandInteraction};
use serenity::prelude::Context;
use serenity::model::prelude::interaction::InteractionResponseType;
use crate::reusable::component::{error::error,button::normal_button};
use crate::reusable::utils::color;

pub async fn run(_options: &[CommandDataOption],ctx:&Context,cmd:&ApplicationCommandInteraction){
    if let Err(why) = cmd.create_interaction_response(&ctx.http, |resp| {
        resp.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|msg|{
                msg.embed(|emb|{
                    emb.title("MHFZ user interface")
                        .color(color("40", "ff", "40"))
                        .description("button interface for mhfz player to make use of server's utility")
                }).components(|c|{
                        c.create_action_row(|r|{
                            r.add_button(normal_button("register", "register_i", ButtonStyle::Primary))
                            .add_button(normal_button("bind", "bind_i", ButtonStyle::Secondary))
                        })
                        .create_action_row(|r|{
                            r.add_button(normal_button("transfer", "transfer_i", ButtonStyle::Primary))
                            .add_button(normal_button("DM save", "dm_save_i", ButtonStyle::Secondary))
                        })
                    })
            })
    }).await{
        error(ctx, &why.to_string(), "interface slash command", "failed to create button, mostlikely connection problem").await;
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("interface").description("An User interface for MHFZ player")
}