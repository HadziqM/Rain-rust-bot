use futures::Future;
use serenity::all::*;
use crate::{Init,MyErr,ErrorLog,ItemPedia, commands};

pub struct MsgBundle<'a>{
    pub ctx:&'a Context,
    pub msg:&'a Message,
    pub init:&'a Init,
    pub pedia:&'a ItemPedia
}

pub async fn msg_handler(ctx:&Context,msg:&Message,init:&Init,pedia:&ItemPedia){
    if msg.content.starts_with(&init.discord.prefix) && !msg.author.bot {
        let name = msg.content.split_whitespace().next();
        let bnd = MsgBundle{ctx,msg,init,pedia};
        if let Some(x) = name{
            let y = x.replace(&init.discord.prefix, "");
            match y.as_str(){
                "test"=>wraper(&bnd, "test commands", test).await,
                "execute"=>wraper(&bnd, "execute postgres", commands::admin::query::msg).await,
                "query"=>wraper(&bnd, "query postgres", commands::admin::query::msg_qry).await,
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
