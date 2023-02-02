use std::time::SystemTime;

use serenity::{prelude::Context, model::{prelude::{ChannelId, Message, UserId}, user::User}, builder::CreateEmbed};
use crate::{Init,PgConn,ErrorLog};
use crate::reusable::postgress::server::Servers;
use crate::reusable::utils::Color;


struct Server<'a>{
    pg:PgConn<'a>,
    err:ErrorLog<'a>,
    msg:Message,
    ctx:&'a Context
}

impl<'a> Server<'a> {
    async fn new(init:&'a Init,ctx:&'a Context,user:&'a User)->Option<Server<'a>>{
        let channel = ChannelId(init.log_channel.info_channel);
        let mut err = ErrorLog::new(ctx, init,user).await;
        err.change_error("idk".to_string(), "paralel loop","dont worry its not important".to_string());
        let msg =match channel.message(&ctx.http, init.log_channel.info_channel_msg).await{
            Ok(x)=>x,
            Err(why)=>{
                println!("cant get message {why}");
                err.change_why(why.to_string());
                err.log_error_channel().await;
                return None;
            }
        };
        let pg =match PgConn::create(init,init.discord.author_id.to_string()).await{
        Ok(x)=>x,
        Err(why)=>{
            println!("failed to perform paralel due {why}");
            err.change_why(why.to_string());
            err.log_error_channel().await;
            return None;
            }
        };
        Some(Server{pg,err,msg,ctx})
    }
    async fn test(&mut self){
        self.msg.edit(&self.ctx.http, |x|{
            x.content(&format!("hello edited <t:{}:R>",SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()))
        }).await.unwrap();
    }
    fn build_embed(&self,serv:Vec<Servers>)->CreateEmbed{
        let mut field = Vec::new();
        let mut pc = 0;
        for x in &serv{
            let z = format!("Description :{}\nPlayer_Count: {}\nLand: {}"
               ,&x.description,x.cp,x.land);
            field.push((&x.name,z,true));
            pc += x.cp
        }
        let mut emb = CreateEmbed::default();
        emb.title("Server Status").description(&format!("Mhfz Server Status updated <t:{}:R>\nPlayer Count Total = {pc}",SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()))
        .fields(field).color(Color::Random.throw());
        emb
    }
    async fn edit_msg(&mut self){
        if let Err(why)=self.pg.re_connect().await{
            self.err.pgcon_error_ch(why.to_string(), "reconnect pgcon").await;
            return;
        }
        let z = match self.pg.get_server().await{
            Ok(x)=>x,
            Err(why)=>{
                self.err.pgcon_error_ch(why.to_string(), "getting server data").await;
                return;
            }
        };
        let embed = self.build_embed(z);
        if let Err(why)=self.msg.edit(&self.ctx.http, |m|{
            m.set_embed(embed)
        }).await{
            self.err.discord_error(why.to_string(), "editing message paralel loop").await;
        }
        self.pg.close().await;
    }
}




pub async fn paralel_thread(ctx:&Context,init:&Init){
    let user = match UserId(init.discord.author_id).to_user(&ctx.http).await{
        Ok(x)=>x,
        Err(why)=>{
            println!("cant get user {why}");
            return;
        }
    };
    let mut serv = match Server::new(init, ctx, &user).await{
        Some(x)=>x,
        None=>{return;}
    };
    serv.edit_msg().await;
}
