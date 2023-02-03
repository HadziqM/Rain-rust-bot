use std::time::Duration;

use serenity::all::{CommandInteraction, ComponentInteraction};
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::futures::StreamExt;
use serenity::prelude::Context;
use crate::reusable::config::Init;


use crate::{PgConn,ErrorLog};
pub struct Register<'a>{
    pub(crate) error:ErrorLog<'a>,
    pub(crate) pg:PgConn<'a>,
    pub(crate) cid:i32
}

impl<'a> Register<'a> {
    pub async fn default(ctx:&'a Context,cmd:&'a CommandInteraction,init:&'a Init,on:&'a str,bypass:bool)->Option<Register<'a>>{
        let mut error = ErrorLog::new(ctx,init,&cmd.user).await;
        let mut pg =match PgConn::create(init,cmd.user.id.to_string()).await{
            Ok(pg)=>pg,
            Err(why)=>{
                error.pgcon_error(why.to_string(), on, cmd).await;
                return None;
            }
        };
        let cards = match pg.get_user_data_long().await{
            Ok(dt)=>{
                if dt.cid != 0 {
                    if !bypass{
                        return Some(Register { error, pg, cid:dt.cid });
                    }
                }else if dt.rid == 0&&dt.cid == 0{
                    pg.close().await;
                    error.change_error("no message".to_string(), "card slash", "you dont have account in this server,try create one".to_string());
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
                error.change_error(why.to_string(), "getting card", "database connection failure".to_string());
                error.log_slash(cmd, false).await;
                pg.close().await;
                return None;
                }
        };
        let mut index:usize = 0;
        if cards.len() == 0{
            error.change_error("You dont have any character in database".to_string(), "getting characters data", "please create character on launcher and safely enter mezeporta befor doing this".to_string());
            error.log_slash(cmd, false).await;
            pg.close().await;
            return None;
        }
        if let Err(why)=cmd.create_response(&ctx.http,cards[index].bind(&cmd.user)).await{
            error.discord_error(why.to_string(), "sending card").await;
            pg.close().await;
            return None;
        }
        let col = match cmd.get_response(&ctx.http).await{
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
                if let Err(why)=cmd.edit_response(&ctx.http,cards[index].edit_bind(&cmd.user)).await{
                    error.discord_error(why.to_string(), "editing the interaction reply").await;
                    break;
                };
                if let Err(why)=pat.defer(&ctx.http).await{
                    error.discord_error(why.to_string(), "defering button update").await;
                    break;
                }
            }else if id=="use"{
                if let Err(why)=pg.switch(cards[index].char_id).await{
                    error.change_error(why.to_string(), "switching char", "report this or try again".to_string());
                    error.log_slash(cmd, false).await;
                    break;
                }
                if let Err(why)=pat.create_response(&ctx.http,CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new().content("succesfully change your main character").ephemeral(true)
                        )).await{
                    error.discord_error(why.to_string(), "replying use button").await;
                };
                break;
            }
        }
        reply.stop();
        if let Err(why)=col.delete(&ctx.http).await{
            error.discord_error(why.to_string(), "deleting bind message").await;
        };
        pg.close().await;
        None
    }
    pub async fn default_button(ctx:&'a Context,cmd:&'a ComponentInteraction,init:&'a Init,on:&'a str)->Option<Register<'a>>{
        let mut error = ErrorLog::new(ctx,init,&cmd.user).await;
        let mut pg =match PgConn::create(init,cmd.user.id.to_string()).await{
            Ok(pg)=>pg,
            Err(why)=>{
                error.pgcon_error_button(why.to_string(), on, cmd).await;
                return None;
            }
        };
        match pg.get_user_data().await{
            Ok(x)=>{
                if x.cid != 0{
                    return Some(Register { error, pg, cid:x.cid });
                }
                error.change_error("you dont have main character".to_string(), "check user data", "please make account if you dont have one or use `/switch` command to select your main character of you already have account".to_string());
                error.log_button(cmd, true).await;
            }
            Err(why)=>{
                error.pgcon_error_button(why.to_string(),on, cmd).await
            }
        }
        pg.close().await;
        None
    }
}
