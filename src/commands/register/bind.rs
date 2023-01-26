use std::time::Duration;

use serenity::builder::CreateApplicationCommand;
use serenity::futures::StreamExt;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use crate::{ErrorLog,Init,PgConn};


#[allow(unused_assignments)]
pub async fn run(ctx:&Context,cmd:&ApplicationCommandInteraction,init:&Init){
    let mut error = ErrorLog::new(ctx,init,&cmd.user).await;
    let cards = match PgConn::create(init, &cmd.user.id.to_string()).await{
        Ok(pg)=>{
            match pg.many_card().await{
                Ok(cd)=>match cd{
                    Some(card)=>card,
                    None=>{
                        error.change_error("no message".to_string(), "card slash", "you dont have account in this server,try create one");
                        error.log_slash(cmd, false).await;
                        return;
                    }
                },
                Err(why)=>{
                    error.change_error(why.to_string(), "getting card", "database connection failure");
                    error.log_slash(cmd, false).await;
                    return;
                }
            }
        }
        Err(why)=>{
            error.change_error(why.to_string(), "getting databse pool", "connection to database timed out, wit for server back to health");
            error.log_slash(cmd, false).await;
            return;
        }
    };
    let mut index:usize = 0;
    if cards.len() == 0{
        error.change_error("You dont have any character in database".to_string(), "getting characters data", "please create character on launcher and safely enter mezeporta befor doing this");
        error.log_slash(cmd, false).await;
        return;
    }
    let path = cards.iter().map(|e|e.get_path().0.to_owned()).collect::<Vec<_>>();
    if let Err(why)=cmd.create_interaction_response(&ctx.http, |m|{
        cards[index].bind(m, &cmd.user, &path)
    }).await{
        error.discord_error(why.to_string(), "sending card").await;
        return;
    }
    let col = match cmd.get_interaction_response(&ctx.http).await{
        Ok(x)=>x,
        Err(why)=>{
            error.discord_error(why.to_string(), "getting interaction msg").await;
            return;
        }
    };
    let mut reply =col.await_component_interactions(ctx).timeout(Duration::from_secs(60*3)).build();
    while let Some(pat) = reply.next().await {
        let id = &pat.data.custom_id;
        if cmd.user != pat.user{
            continue;
        }
        if id=="next"{
            index += 1;
            if index==cards.len(){
                index = 0;
            }
            if let Err(why)=cmd.edit_original_interaction_response(&ctx.http, |f|{
                cards[index].edit_bind(f, &cmd.user)
            }).await{
                error.discord_error(why.to_string(), "editing the interaction reply").await;
                break;
            };
            if let Err(why)=pat.defer(&ctx.http).await{
                error.discord_error(why.to_string(), "defering button update").await;
                break;
            }
        }else if id=="use"{
            if let Err(why)=pat.create_interaction_response(&ctx.http, |m|{
                m.kind(InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|msg|msg.content("ok selected"))
            }).await{
                error.discord_error(why.to_string(), "replying use button").await;
            };
            if let Err(why)=col.delete(&ctx.http).await{
                error.discord_error(why.to_string(), "deleting bind message").await;
            };
            break;
        }
    }
    reply.stop();
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("switch").description("switch your own binded character for server event")
}

