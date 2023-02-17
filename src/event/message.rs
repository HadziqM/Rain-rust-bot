use futures::Future;
use serenity::all::*;
use crate::{Init,MyErr,ErrorLog,ItemPedia};

pub struct MsgBundle<'a>{
    ctx:&'a Context,
    msg:&'a Message,
    init:&'a Init,
    pedia:&'a ItemPedia
}

pub async fn msg_handler(ctx:&Context,msg:&Message,init:&Init,pedia:&ItemPedia){
    if msg.content.starts_with(&init.discord.prefix) && !msg.author.bot {
        let name = msg.content.split(&init.discord.prefix).next();
        let bnd = MsgBundle{ctx,msg,init,pedia};
        if let Some(x) = name{
            match x{
                "test"=>wraper(&bnd, "test commands", test).await,
                _=>{return ;}
            }
        }
    }
}
async fn wraper<'a,'b,F:Fn(&'a MsgBundle<'b>)->Fut,Fut:Future<Output = Result<(),MyErr>>>(bnd:&'a MsgBundle<'b>,on:&'static str,f:F){
    let mut err = ErrorLog::new(&bnd.ctx,&bnd.init,&bnd.msg.author).await;
    if let Err(why)=f(bnd).await{
        why.log_msg(&bnd.msg, on, &mut err).await;
    }
}
async fn test(bnd:&MsgBundle<'_>)->Result<(),MyErr>{
    bnd.msg.channel_id.send_message(&bnd.ctx.http,CreateMessage::new().embed(CreateEmbed::new().title("tested"))).await?;
    Ok(())
}
