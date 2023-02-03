use std::time::SystemTime;

use serenity::{prelude::Context, model::{prelude::{ChannelId, Message, UserId}, user::User}, builder::{CreateEmbed, EditMessage}};
use crate::{Init,PgConn,ErrorLog};
use crate::reusable::postgress::server::Servers;
use crate::reusable::utils::Color;
use super::log::logging;


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
    fn build_embed(&self,serv:Vec<Servers>)->CreateEmbed{
        let mut field = Vec::new();
        let mut pc = 0;
        for x in &serv{
            let z = format!("Description :{}\nPlayer_Count: {}\nLand: {}"
               ,&x.description,x.cp,x.land);
            field.push((&x.name,z,true));
            pc += x.cp
        }
        let mut emb = CreateEmbed::new();
        emb.title("Server Status").description(&format!("Mhfz Server Status updated <t:{}:R>\nPlayer Count Total = {pc}",SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()))
        .fields(field).color(Color::Random.throw());
        emb
    }
    async fn edit_msg(&mut self)->i32{
        let z = match self.pg.get_server().await{
            Ok(x)=>x,
            Err(why)=>{
                self.err.pgcon_error_ch(why.to_string(), "getting server data").await;
                return 0;
            }
        };
        let pc  = z.iter().map(|e|e.cp).sum::<i32>();
        let embed = self.build_embed(z);
        if let Err(why)=self.msg.edit(&self.ctx.http,EditMessage::new().embed(embed)).await{
            self.err.discord_error(why.to_string(), "editing message paralel loop").await;
        }
        self.pg.close().await;
        pc
    }
}

static mut CURRENT_PLAYER:i32 = 0;

//event handler will spawn a thread calling this function every 5 minutes
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
    let cp;
    //run function to update server status on info channel
    let now = serv.edit_msg().await;
    //sorry to use unsafe, since rust doesnt support global variable
    //and this thread is completely detached from main thread
    //so we cant get access to Handler struct
    unsafe{
        cp = CURRENT_PLAYER;
        CURRENT_PLAYER = now;
    }
    //if the current player is 0 and is not 0 before, announce server crash
    //and dm wish the log
    let wish;
    if now==0 && cp != 0{
        wish = Some(UserId(119094696487288833).to_user(&ctx.http).await.unwrap());
    }else{
        wish = None
    }
    //execute the logging logic
    logging(ctx,init,wish).await;
}
