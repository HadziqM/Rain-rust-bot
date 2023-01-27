use std::time::Duration;

use serenity::futures::StreamExt;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::{prelude::Context, model::prelude::interaction::application_command::ApplicationCommandInteraction};

use crate::reusable::config::Init;


use crate::{PgConn,ErrorLog};
pub struct Register<'a>{
    pub(crate) error:ErrorLog<'a>,
    pub(crate) pg:PgConn<'a>,
    pub(crate) cid:i32
}

impl<'a> Register<'a> {
    pub async fn default(ctx:&'a Context,cmd:&'a ApplicationCommandInteraction,init:&'a Init,did:&'a str,on:&'a str)->Option<Register<'a>>{
        let mut error = ErrorLog::new(ctx,init,&cmd.user).await;
        let mut pg =match PgConn::create(init,&did).await{
            Ok(pg)=>pg,
            Err(why)=>{
                error.pgcon_error(why.to_string(), on, cmd).await;
                return None;
            }
        };
        let cards = match pg.get_user_data().await{
            Ok(dt)=>{
                if dt.cid != 0 {
                    return Some(Register { error, pg, cid:dt.cid });
                }else if dt.rid == 0{
                    pg.close().await;
                    error.change_error("no message".to_string(), "card slash", "you dont have account in this server,try create one");
                    error.log_slash(cmd, false).await;
                    return None;
                }
                match pg.many_card(dt.rid).await{
                    Ok(card)=>card,
                    Err(why)=>{
                        error.pgcon_error(why.to_string(),"getting user card", cmd).await;
                        pg.close().await;
                        return None;
                    }
                }
            }
            Err(why)=>{
                error.change_error(why.to_string(), "getting card", "database connection failure");
                error.log_slash(cmd, false).await;
                pg.close().await;
                return None;
                }
        };
        let mut index:usize = 0;
        if cards.len() == 0{
            error.change_error("You dont have any character in database".to_string(), "getting characters data", "please create character on launcher and safely enter mezeporta befor doing this");
            error.log_slash(cmd, false).await;
            pg.close().await;
            return None;
        }
        let path = cards.iter().map(|e|e.get_path().0.to_owned()).collect::<Vec<_>>();
        if let Err(why)=cmd.create_interaction_response(&ctx.http, |m|{
            cards[index].bind(m, &cmd.user, &path)
        }).await{
            error.discord_error(why.to_string(), "sending card").await;
            pg.close().await;
            return None;
        }
        let col = match cmd.get_interaction_response(&ctx.http).await{
            Ok(x)=>x,
            Err(why)=>{
                error.discord_error(why.to_string(), "getting interaction msg").await;
                pg.close().await;
                return None;
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
        pg.close().await;
        None
    }
}
