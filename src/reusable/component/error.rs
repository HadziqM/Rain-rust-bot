use serenity::builder::CreateInteractionResponse;
use serenity::model::prelude::interaction::message_component::MessageComponentInteraction;
use serenity::model::prelude::interaction::modal::ModalSubmitInteraction;
use serenity::model::prelude::{ChannelId, UserId};
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::user::User;
use serenity::prelude::Context;


use super::super::config::Init;
use super::super::utils::color;



pub async fn error(ctx:&Context,err:&str,on:&str,advice:&str,init:&Init,usr:&User){
    let ch_id = ChannelId(init.log_channel.err_channel.to_owned());
    let user = UserId(init.discord.author_id).to_user(&ctx.http).await.unwrap_or_default();
    if let Err(why) = ch_id.send_message(&ctx.http, |msg|{
        msg.content(&format!("for {}",usr.to_string())).embed(|emb|{
            emb.title("🛑 Error Occured 🛑")
                .description("some cant be handled error occured")
                .fields(vec![
                    ("🚧 occured on",on,false),
                    ("📜 error message",&format!("```\n{err}\n```"),false),
                    ("⛑  author advice",advice,false)
                ])
                .author(|f|f.name(usr.name.as_str()).icon_url(usr.face()))
                .footer(|f|f.text(format!("you can consult this to {}",user.tag()))
                    .icon_url(user.face()))
                .color(color("ff", "00", "00"))
                .thumbnail("attachment://panics.png")
        }).add_file("./icon/panics.png")
    }).await{
        println!("cant send error message to discord channel :{}",why)
    }
}

fn error_reply<'a,'b>(m:&'a mut CreateInteractionResponse<'b>,usr:&User,user:&User,on:&str,advice:&str,err:&str)->&'a mut CreateInteractionResponse<'b>{
    m.kind(InteractionResponseType::ChannelMessageWithSource)
    .interaction_response_data(|msg|{
            msg.add_file("./icon/panics.png").embed(|emb|{
            emb.title("🛑 Error Occured 🛑")
                .description("some cant be handled error occured")
                .fields(vec![
                    ("🚧 occured on",on,false),
                    ("📜 error message",&format!("```\n{err}\n```"),false),
                    ("⛑  author advice",advice,false)
                ])
                .author(|f|f.name(usr.name.as_str()).icon_url(usr.face()))
                .footer(|f|f.text(format!("you can consult this to {}",user.tag()))
                    .icon_url(user.face()))
                .color(color("ff", "00", "00"))
                .thumbnail("attachment://panics.png")
            })
        })
}
pub async fn error_interaction(ctx:&Context,err:&str,on:&str
    ,advice:&str,cmd:&ApplicationCommandInteraction,init:&Init){
    let usr = &cmd.user;
    let user = UserId(init.discord.author_id).to_user(&ctx.http).await.unwrap_or_default();
    if let Err(why) = cmd.create_interaction_response(&ctx.http, |m|
        error_reply(m,usr,&user,on,advice,err)).await{
        error(ctx, why.to_string().as_str(), "sending error msg"
            , "just discord connection problem",init,usr).await;
        println!("{why}");
    }
}
pub async fn error_modal(ctx:&Context,err:&str,on:&str
    ,advice:&str,cmd:&ModalSubmitInteraction,init:&Init){
    let usr = &cmd.user;
    let user = UserId(init.discord.author_id).to_user(&ctx.http).await.unwrap_or_default();
    if let Err(why) = cmd.create_interaction_response(&ctx.http, |m|
        error_reply(m,usr,&user,on,advice,err)).await{
        error(ctx, why.to_string().as_str(), "sending error msg"
            , "just discord connection problem",init,usr).await;
        println!("{why}");
    }
}
pub async fn error_button(ctx:&Context,err:&str,on:&str
    ,advice:&str,cmd:&MessageComponentInteraction,init:&Init){
    let usr = &cmd.user;
    let user = UserId(init.discord.author_id).to_user(&ctx.http).await.unwrap_or_default();
    if let Err(why) = cmd.create_interaction_response(&ctx.http, |m|
        error_reply(m,usr,&user,on,advice,err)).await{
        error(ctx, why.to_string().as_str(), "sending error msg"
            , "just discord connection problem",init,usr).await;
        println!("{why}");
    }
}
