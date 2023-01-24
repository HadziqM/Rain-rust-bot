use serenity::builder::CreateActionRow;
use serenity::model::prelude::component::{InputTextStyle, ActionRowComponent};
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::interaction::message_component::MessageComponentInteraction;
use serenity::model::application::interaction::modal::{ModalSubmitInteraction,ModalSubmitInteractionData};
use serenity::prelude::Context;

use crate::reusable::config::Init;
use crate::reusable::component::error::error;

fn modal_register_row(name:&str,pass:bool)->CreateActionRow{
    let placeholder = match pass {
        false => "your MHFZ username on launcher".to_owned(),
        true => "your MHFZ user password (igonore discord warning)".to_owned(),
    };
    let mut row = CreateActionRow::default();
    row.create_input_text(|i|{
        i.label(name)
         .custom_id(name)
         .required(true)
         .style(InputTextStyle::Short)
         .placeholder(&placeholder)
    });
    row
}

pub async fn register(ctx:&Context,cmd:&MessageComponentInteraction,init:&Init){
    if let Err(why) = cmd.create_interaction_response(&ctx.http, |r|{
        r.kind(InteractionResponseType::Modal)
        .interaction_response_data(|m|{
                m.components(|c|c.add_action_row(modal_register_row("username",false))
                   .add_action_row(modal_register_row("password", true)))
                    .custom_id("register_m")
                    .title("register command")
            })
    }).await{
        error(ctx, &why.to_string(), "register interface button", "connection to database problem or ghost data input, either way just consult",init,&cmd.user).await;
    }
}

pub async fn modal_register(ctx:&Context,cmd:&ModalSubmitInteraction,data:&ModalSubmitInteractionData,init:&Init){
    let mut name = String::new();
    let mut password = String::new();
    for comp in &data.components{
        let arow = comp.components.first().unwrap();
        if let ActionRowComponent::InputText(input) = arow{
            match input.custom_id.as_str() {
                "username" => name = input.value.to_owned(),
                 _=> password = input.value.to_owned(),
            }
        }
    }
    if let Err(why) = cmd.create_interaction_response(&ctx.http, |r|{
        r.kind(InteractionResponseType::ChannelMessageWithSource)
        .interaction_response_data(|m|{
                m.ephemeral(true).content(&format!("username :{}\npass : {}",name,password))
            })
    }).await{
        error(ctx, &why.to_string(), "register modal response", "discord connection failure, make them try again",init,&cmd.user).await;
        println!("{why}")
    };
}
// pub async fn dm_save(ctx:&Context,cmd:&MessageComponentInteraction){
//     let mut message = "successfullu send dm".to_string();
//     match tokio::fs::File::open("./README.md").await{
//         Ok(file)=>{
//             if let Err(why)=cmd.user.direct_message(&ctx.http, |m|{
//                 m.content("this is yor file").add_file(AttachmentType::File { file:&file
//                     , filename: "readme.md".to_string() })
//             }).await{
//                 error(ctx, &why.to_string(),&format!("dm {}",cmd.user.name), "make sure they have dm enabled").await;
//                 println!("{why}");
//                 message = "send failed, please enable your direct message".to_owned();
//             }
//         }
//         Err(err)=>{
//             error(ctx, &err.to_string(), "read file", "make sure file is exist").await;
//             message = "our server encounter a problem, wait few minute or ask owner".to_string()
//         }
//     };
//     if let Err(why) = cmd.create_interaction_response(&ctx.http, |r|{
//         r.kind(InteractionResponseType::ChannelMessageWithSource)
//         .interaction_response_data(|m|{
//                 m.content(&message).ephemeral(true)
//             })
//     }).await{
//         error(ctx, &why.to_string(), "register interface button", "connection to database problem or ghost data input, either way just consult").await;
//         println!("{why}");
//     }
// }
// pub async fn bind(ctx:&Context,cmd:&MessageComponentInteraction){
//     if let Err(why) = cmd.create_interaction_response(&ctx.http, |r|{
//         r.kind(InteractionResponseType::ChannelMessageWithSource)
//         .interaction_response_data(|m|{
//                 m.content("pushed create button")
//         })
//     }).await{
//         error(ctx, &why.to_string(), "register interface button", "connection to database problem or ghost data input, either way just consult").await;
//     }
// }
// pub async fn transfer(ctx:&Context,cmd:&MessageComponentInteraction){
//     if let Err(why) = cmd.create_interaction_response(&ctx.http, |r|{
//         r.kind(InteractionResponseType::ChannelMessageWithSource)
//         .interaction_response_data(|m|{
//                 //todo create register command
//                 m.content("pushed create button")
//             })
//     }).await{
//         error(ctx, &why.to_string(), "register interface button", "connection to database problem or ghost data input, either way just consult").await;
//     }
// }
