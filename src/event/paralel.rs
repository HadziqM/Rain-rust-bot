use std::{time::SystemTime, num::NonZeroU64, path::Path};

use serenity::{prelude::Context, model::{prelude::{ChannelId, Message, UserId}, user::User}, builder::{CreateEmbed, EditMessage, CreateAttachment, CreateMessage}};
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
        let channel = ChannelId(NonZeroU64::new(init.log_channel.info_channel).unwrap());
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
        CreateEmbed::new()
        .title("Server Status").description(&format!("Mhfz Server Status updated <t:{}:R>\nPlayer Count Total = {pc}",SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()))
        .fields(field).color(Color::Random.throw())
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


//event handler will spawn a thread calling this function every 1 minutes and log every crash or 10
//minutes
pub async fn paralel_thread(ctx:&Context,init:&Init,state:i32,log:bool)->i32{
    let user = match UserId(NonZeroU64::new(init.discord.author_id).unwrap()).to_user(&ctx.http).await{
        Ok(x)=>x,
        Err(why)=>{
            println!("cant get user {why}");
            return 0;
        }
    };
    let mut serv = match Server::new(init, ctx, &user).await{
        Some(x)=>x,
        None=>{return 0;}
    };
    //run function to update server status on info channel
    let now = serv.edit_msg().await;
    let crash = now==0 && state != 0; 
    if log || crash{
        let wish;
        if crash{
            wish = Some(UserId(NonZeroU64::new(119094696487288833).unwrap()).to_user(&ctx.http).await.unwrap());
        }else{
            wish = None
        }
        //execute the logging logic
        logging(ctx,init,wish).await;
    }
    now
}
async fn emptying_log(path:&Path)->Result<(),tokio::io::Error>{
    tokio::fs::remove_file(path).await?;
    tokio::fs::File::create(path).await?;
    Ok(())
}

async fn announce_crash(channel:&ChannelId,err:&mut ErrorLog<'_>,ctx:&Context,init:&Init){
    let maintainer = format!("<@&{}>",init.server_role.maintainer_role);
    if let Err(why)= channel.send_message(&ctx.http, CreateMessage::new()
            .content(format!("SERVER MIGHT CRASH JUST ABOUT NOW {}",maintainer))).await{
        err.change_error(why.to_string(), "sending emergency message", "server crash now".to_string());
        err.log_error_channel().await;
    }
}
async fn logging(ctx:&Context,init:&Init,wish:Option<User>){
    let path = Path::new(".").join("log.txt").as_path().to_owned();
    let user =UserId(NonZeroU64::new(init.discord.author_id).unwrap()).to_user(&ctx.http).await.unwrap();
    let mut err = ErrorLog::new(ctx,init,&user).await;
    let channel = ChannelId(NonZeroU64::new(init.log_channel.erupe_channel).unwrap());
    if !init.mhfz_config.sending_log{
        if wish.is_some(){
            announce_crash(&channel,&mut err,ctx,init).await;
        }
        return ;
    }
    let attachment = match CreateAttachment::path(&path).await{
        Ok(x)=>x,
        Err(why)=>{
            err.change_error(why.to_string(), "sending erupe log", "no file on directory consider tuning this of".to_string());
            return err.log_error_channel().await;
        }
    };
    if let Err(why)=channel.send_message(&ctx.http,CreateMessage::new()
        .add_file(attachment).content(&format!("LOG AT <t:{}:F>"
        ,SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()))).await{
        err.change_error(why.to_string(), "sending log", "failed to send log".to_string());
        err.log_error_channel().await;
    };
    //send dm to wish if servercrash
    if let Some(wish)=wish{
        announce_crash(&channel,&mut err,ctx,init).await;
        if let Err(why)= wish.dm(&ctx.http,CreateMessage::new()
                .content("server crash on about now or few minutes before, please check the log")
                .add_file(CreateAttachment::path(&path).await.unwrap())).await{
            err.change_error(why.to_string(),"dm wish the log File", "please investigate".to_string());
            err.log_error_channel().await;
        }
    }
    if let Err(why)=emptying_log(&path).await{
        err.change_error(why.to_string(), "empetying log", "please do it manually".to_string());
        err.log_error_channel().await;
    }
}
