use serenity::model::prelude::component::ButtonStyle;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use serenity::model::prelude::interaction::InteractionResponseType;
use crate::{Init,ErrorLog,Components};
use crate::reusable::utils::color;

pub async fn run(ctx:&Context,cmd:&ApplicationCommandInteraction,init:&Init){
    if let Err(why) = cmd.create_interaction_response(&ctx.http, |resp| {
        resp.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|msg|{
                msg.embed(|emb|{
                    emb.title("MHFZ user interface")
                        .color(color("40", "ff", "40"))
                        .description("button interface for mhfz player to make use of server's utility")
                }).components(|c|{
                        c.create_action_row(|r|{
                            r.add_button(Components::normal_button("register", "register", ButtonStyle::Primary,"📝"))
                            .add_button(Components::normal_button("DM Save", "dms", ButtonStyle::Secondary,"🔐"))
                        })
                    })
            })
    }).await{
        let mut err = ErrorLog::new(&ctx, init, &cmd.user).await;
        err.discord_error(why.to_string(), "interface command").await;
    }
}
