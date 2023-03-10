use std::time::Duration;

use serenity::all::User;
use serenity::futures::StreamExt;
use serenity::prelude::Context;
use crate::{Init,MyErr,Mytrait,PgConn,Components,Mybundle};

pub struct Reg<'a>{
    pub pg:PgConn<'a>,
    pub cid:i32
}
impl<'a> Reg<'a>{
    pub async fn switch<T:Mytrait>(ctx:&'a Context,cmd:&'a T,init:&'a Init,bypass:bool,ephemeral:bool)->Result<Option<Reg<'a>>,MyErr>{
        let user = cmd.user();
        let pg = PgConn::create(init, user.id.to_string()).await?;
        let data = pg.get_user_data_long().await?;
        if data.cid != 0 && !bypass{
            return Ok(Some(Reg { pg, cid:data.cid }));
        }else if data.rid == 0 && data.cid == 0{
            return Err(MyErr::Custom("you doesnt have any account in this server, please create account with `/create` or bind existing one with `/bind`".to_string()));
        }
        let card = pg.many_card(data.rid).await?;
        if card.len() == 0{
            return Err(MyErr::Custom("you doesnt have any charachter on your account, please make one on the game launcher and use it to enter town first before using this command again".to_string()));
        }
        let mut index = 0;
        cmd.response(ctx, card[index].bind(&user)).await?;
        let msg = cmd.get_msg(ctx).await?;
        let mut reply = msg.await_component_interactions(ctx).timeout(Duration::from_secs(60*3)).stream();
        while let Some(pat) = reply.next().await {
            let id = &pat.data.custom_id;
            if user != pat.user{
                continue;
            }
            if id=="next"{
                index += 1;
                if index==card.len(){
                    index = 0;
                }
                cmd.edit(ctx,card[index].edit_bind(&user)).await?;
                pat.defer(&ctx.http).await?;
            }else if id=="use"{
                pg.switch(card[index].char_id).await?;
                pat.create_response(&ctx.http,Components::interaction_response("successfully switch main character", ephemeral)).await?;
                break;
            }
        }
        msg.delete(&ctx.http).await?;
        Ok(None)
    }
    pub async fn check<T:Mybundle>(bnd:&'a T,user:&'a User)->Result<Reg<'a>,MyErr>{
        let pg = PgConn::create(bnd.init(), user.id.to_string()).await?;
        let data = pg.get_user_data().await?;
        if data.cid == 0{
            if data.rid ==0{
                return Err(MyErr::Custom(format!("{} doesnt have account in this server yet, use `/create` to create new account or bind existing account with `/bind`",user.to_string())));
            }else{
                return Err(MyErr::Custom(format!("{} already have account but doesnt have main character selected, try tell them to use any command or `/switch` to select their main character if they have any characters in their account, otherwise tell them to create characters in game first ",user.to_string())));
            }
        }
        Ok(Reg{pg,cid:data.cid})
    }
    pub async fn check_user<T:Mybundle>(bnd:&'a T,user:&'a User)->Result<Reg<'a>,MyErr>{
        let pg = PgConn::create(bnd.init(), user.id.to_string()).await?;
        let data = pg.get_user_data_long().await?;
        if data.rid == 0{
            return Err(MyErr::Custom(format!("{} doesnt have account in this server yet, use `/create` to create new account or bind existing account with `/bind`",user.to_string())));
        }
        Ok(Reg{pg,cid:data.cid})
    }
    pub async fn reverse_check<T:Mybundle>(bnd:&'a T,user:&'a User)->Result<Reg<'a>,MyErr>{
        let pg = PgConn::create(bnd.init(), user.id.to_string()).await?;
        let data = pg.get_user_data_long().await?;
        if data.rid != 0 {
            return Err(MyErr::Custom(format!("{} already have account in this server,use `/check` or `/change_password` if you forgot your account name or password, or try use `/switch` command instead if you already have characters in game on that account and want to switch characters",user.to_string())));
        }
        Ok(Reg{pg,cid:data.cid})
    }
}
