use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::interaction::message_component::MessageComponentInteraction;
use serenity::prelude::Context;

use crate::reusable::component::error::error;

pub async fn register(ctx:&Context,cmd:&MessageComponentInteraction){
    if let Err(why) = cmd.create_interaction_response(&ctx.http, |r|{
        r.kind(InteractionResponseType::ChannelMessageWithSource)
        .interaction_response_data(|m|{
                //todo create register command
                m.content(&format!("{} pushed register button",cmd.user.name))
            })
    }).await{
        error(ctx, &why.to_string(), "register interface button", "connection to database problem or ghost data input, either way just consult").await;
    }
}
pub async fn dm_save(ctx:&Context,cmd:&MessageComponentInteraction){
    if let Err(why) = cmd.create_interaction_response(&ctx.http, |r|{
        r.kind(InteractionResponseType::ChannelMessageWithSource)
        .interaction_response_data(|m|{
                //todo create register command
                m.content("pushed create button")
            })
    }).await{
        error(ctx, &why.to_string(), "register interface button", "connection to database problem or ghost data input, either way just consult").await;
    }
}
pub async fn bind(ctx:&Context,cmd:&MessageComponentInteraction){
    if let Err(why) = cmd.create_interaction_response(&ctx.http, |r|{
        r.kind(InteractionResponseType::ChannelMessageWithSource)
        .interaction_response_data(|m|{
                //todo create register command
                m.content("pushed create button")
            })
    }).await{
        error(ctx, &why.to_string(), "register interface button", "connection to database problem or ghost data input, either way just consult").await;
    }
}
pub async fn transfer(ctx:&Context,cmd:&MessageComponentInteraction){
    if let Err(why) = cmd.create_interaction_response(&ctx.http, |r|{
        r.kind(InteractionResponseType::ChannelMessageWithSource)
        .interaction_response_data(|m|{
                //todo create register command
                m.content("pushed create button")
            })
    }).await{
        error(ctx, &why.to_string(), "register interface button", "connection to database problem or ghost data input, either way just consult").await;
    }
}
