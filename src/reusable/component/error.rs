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

pub struct ErrorLog<'a> {
    pub(crate) err: String,
    pub(crate) on:&'a str,
    pub(crate) advice:&'a str,
    pub(crate) ctx:&'a Context,
    pub(crate) init:&'a Init,
    pub(crate) usr:&'a User,
    pub(crate) user:User,
}

impl<'a> ErrorLog<'a>{
    pub async fn new(ctx:&'a Context, init:&'a Init,usr:&'a User)->ErrorLog<'a>{
        let user = UserId(init.discord.author_id).to_user(&ctx.http).await.unwrap_or_default();
        ErrorLog { 
            err: String::new(), 
            on: "", advice: "", 
            ctx, 
            init,
            usr,
            user
        }
    }
    pub fn change_error(&mut self,error:String,on:&'a str,advice:&'a str){
        self.err = error;
        self.on = on;
        self.advice = advice;
    }
    pub async fn log_error_channel(&self){
        let ch_id = ChannelId(self.init.log_channel.err_channel);
        let user = UserId(self.init.discord.author_id).to_user(&self.ctx.http).await.unwrap_or_default();
        if let Err(why) = ch_id.send_message(&self.ctx.http, |msg|{
            msg.content(&format!("for {}",self.usr.to_string())).embed(|emb|{
                emb.title("🛑 Error Occured 🛑")
                    .description("some cant be handled error occured")
                    .fields(vec![
                        ("🚧 occured on",self.on,false),
                        ("📜 error message",&format!("```\n{}\n```",&self.err),false),
                        ("⛑  author advice",self.advice,false)
                    ])
                    .author(|f|f.name(self.usr.name.as_str()).icon_url(self.usr.face()))
                    .footer(|f|f.text(format!("you can consult this to {}",user.tag()))
                        .icon_url(user.face()))
                    .color(color("ff", "00", "00"))
                    .thumbnail("attachment://panics.png")
            }).add_file("./icon/panics.png")
        }).await{
            println!("cant send error message to discord channel :{}",why)
        }
    }
    fn interaction_response<'b,'c>(&self,m:&'c mut CreateInteractionResponse<'b>,ephemeral:bool)-> 
    &'c mut CreateInteractionResponse<'b>{
        m.kind(InteractionResponseType::ChannelMessageWithSource)
        .interaction_response_data(|msg|{
                msg.ephemeral(ephemeral).add_file("./icon/panics.png").embed(|emb|{
                emb.title("🛑 Error Occured 🛑")
                    .description("some cant be handled error occured")
                    .fields(vec![
                        ("🚧 occured on",self.on,false),
                        ("📜 error message",&format!("```\n{}\n```",&self.err),false),
                        ("⛑  author advice",self.advice,false)
                    ])
                    .author(|f|f.name(self.usr.name.as_str()).icon_url(self.usr.face()))
                    .footer(|f|f.text(format!("you can consult this to {}",&self.user.tag()))
                        .icon_url(&self.user.face()))
                    .color(color("ff", "00", "00"))
                    .thumbnail("attachment://panics.png")
                })
            })
    }
    pub async fn log_slash(&mut self,cmd:&ApplicationCommandInteraction,ephemeral:bool){
        if let Err(why) = cmd.create_interaction_response(&self.ctx.http, |m|
            self.interaction_response(m,ephemeral)).await{
            self.change_error(why.to_string(),"sending error msg", "discord connection problem");
            self.log_error_channel().await;
            println!("{why}");
        }
    }
    pub async fn log_button(&mut self,cmd:&MessageComponentInteraction,ephemeral:bool){
        if let Err(why) = cmd.create_interaction_response(&self.ctx.http, |m|
            self.interaction_response(m,ephemeral)).await{
            self.change_error(why.to_string(),"sending error msg", "discord connection problem");
            self.log_error_channel().await;
            println!("{why}");
        }
    }
    pub async fn log_modal(&mut self,cmd:&ModalSubmitInteraction,ephemeral:bool){
        if let Err(why) = cmd.create_interaction_response(&self.ctx.http, |m|
            self.interaction_response(m,ephemeral)).await{
            self.change_error(why.to_string(),"sending error msg", "discord connection problem");
            self.log_error_channel().await;
            println!("{why}");
        }
    }

}
// pub async fn error(ctx:&Context,err:&str,on:&str,advice:&str,init:&Init,usr:&User){
//     let ch_id = ChannelId(init.log_channel.err_channel.to_owned());
//     let user = UserId(init.discord.author_id).to_user(&ctx.http).await.unwrap_or_default();
//     if let Err(why) = ch_id.send_message(&ctx.http, |msg|{
//         msg.content(&format!("for {}",usr.to_string())).embed(|emb|{
//             emb.title("🛑 Error Occured 🛑")
//                 .description("some cant be handled error occured")
//                 .fields(vec![
//                     ("🚧 occured on",on,false),
//                     ("📜 error message",&format!("```\n{err}\n```"),false),
//                     ("⛑  author advice",advice,false)
//                 ])
//                 .author(|f|f.name(usr.name.as_str()).icon_url(usr.face()))
//                 .footer(|f|f.text(format!("you can consult this to {}",user.tag()))
//                     .icon_url(user.face()))
//                 .color(color("ff", "00", "00"))
//                 .thumbnail("attachment://panics.png")
//         }).add_file("./icon/panics.png")
//     }).await{
//         println!("cant send error message to discord channel :{}",why)
//     }
// }
//
// fn error_reply<'a,'b>(m:&'a mut CreateInteractionResponse<'b>,usr:&User,user:&User,on:&str,advice:&str,err:&str)->&'a mut CreateInteractionResponse<'b>{
//     m.kind(InteractionResponseType::ChannelMessageWithSource)
//     .interaction_response_data(|msg|{
//             msg.add_file("./icon/panics.png").embed(|emb|{
//             emb.title("🛑 Error Occured 🛑")
//                 .description("some cant be handled error occured")
//                 .fields(vec![
//                     ("🚧 occured on",on,false),
//                     ("📜 error message",&format!("```\n{err}\n```"),false),
//                     ("⛑  author advice",advice,false)
//                 ])
//                 .author(|f|f.name(usr.name.as_str()).icon_url(usr.face()))
//                 .footer(|f|f.text(format!("you can consult this to {}",user.tag()))
//                     .icon_url(user.face()))
//                 .color(color("ff", "00", "00"))
//                 .thumbnail("attachment://panics.png")
//             })
//         })
// }
// pub async fn error_interaction(ctx:&Context,err:&str,on:&str
//     ,advice:&str,cmd:&ApplicationCommandInteraction,init:&Init){
//     let usr = &cmd.user;
//     let user = UserId(init.discord.author_id).to_user(&ctx.http).await.unwrap_or_default();
//     if let Err(why) = cmd.create_interaction_response(&ctx.http, |m|
//         error_reply(m,usr,&user,on,advice,err)).await{
//         error(ctx, why.to_string().as_str(), "sending error msg"
//             , "just discord connection problem",init,usr).await;
//         println!("{why}");
//     }
// }
// pub async fn error_modal(ctx:&Context,err:&str,on:&str
//     ,advice:&str,cmd:&ModalSubmitInteraction,init:&Init){
//     let usr = &cmd.user;
//     let user = UserId(init.discord.author_id).to_user(&ctx.http).await.unwrap_or_default();
//     if let Err(why) = cmd.create_interaction_response(&ctx.http, |m|
//         error_reply(m,usr,&user,on,advice,err)).await{
//         error(ctx, why.to_string().as_str(), "sending error msg"
//             , "just discord connection problem",init,usr).await;
//         println!("{why}");
//     }
// }
// pub async fn error_button(ctx:&Context,err:&str,on:&str
//     ,advice:&str,cmd:&MessageComponentInteraction,init:&Init){
//     let usr = &cmd.user;
//     let user = UserId(init.discord.author_id).to_user(&ctx.http).await.unwrap_or_default();
//     if let Err(why) = cmd.create_interaction_response(&ctx.http, |m|
//         error_reply(m,usr,&user,on,advice,err)).await{
//         error(ctx, why.to_string().as_str(), "sending error msg"
//             , "just discord connection problem",init,usr).await;
//         println!("{why}");
//     }
// }
