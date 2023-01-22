use serenity::builder::CreateActionRow;
use serenity::model::prelude::component::InputTextStyle;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::interaction::message_component::MessageComponentInteraction;
use serenity::prelude::Context;

use crate::reusable::component::error::error;

fn modal_register_row()->CreateActionRow{
    let mut row = CreateActionRow::default();
    row.create_input_text(|i|{
        i.label("username")
         .custom_id("username")
         .required(true)
         .style(InputTextStyle::Short)
         .placeholder("your MHFZ username on launcher")
    }).create_input_text(|i|{
        i.label("password")
         .custom_id("password")
         .required(true)
         .style(InputTextStyle::Short)
         .placeholder("your MHFZ user password (igonore discord warning)")
        });
    row
}

pub async fn register(ctx:&Context,cmd:&MessageComponentInteraction){
    if let Err(why) = cmd.create_interaction_response(&ctx.http, |r|{
        r.kind(InteractionResponseType::Modal)
        .interaction_response_data(|m|{
                m.components(|c|c.add_action_row(modal_register_row()))
            })
    }).await{
        error(ctx, &why.to_string(), "register interface button", "connection to database problem or ghost data input, either way just consult").await;
    }
}


pub async fn dm_save(ctx:&Context,cmd:&MessageComponentInteraction){
    //todo download save from db and send it
    if let Err(why) = cmd.user.direct_message(&ctx.http, |m|{
        m.content("your save file")
    }).await{
        error(ctx, &why.to_string(), "direct message savefile", "make sure user enable direct message, or connecttion to database is established").await;
    };
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
