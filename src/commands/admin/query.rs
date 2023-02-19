use std::num::NonZeroU64;

use serenity::all::*;
use crate::{MsgBundle,MyErr,PgConn,Components};

struct Code{
    code:String
}
impl Default for Code {
    fn default() -> Self {
        Code { code: String::new() }
    }
}
impl Code{
    fn new(cont:String)->Result<Code,MyErr>{
        let mut code = Vec::new();
        let mut state = false;
        while let Some(x) = cont.split("\n").next(){
            if state{
                code.push(x);
            }
            if x.starts_with("```"){
                state = !state;
            }
        }
        if code.len() == 0{
            return Err(MyErr::Custom("cant detect code in your message".to_owned()))
        }
        Ok(Code{code:code.concat()})
    }
}

pub async fn msg(bnd:&MsgBundle<'_>)->Result<(),MyErr>{
    if !bnd.msg.author.has_role(&bnd.ctx.http,bnd.msg.guild_id.unwrap().to_owned(), RoleId(NonZeroU64::new(bnd.init.server_role.admin_role).unwrap())).await?{
        return Err(MyErr::Custom("You dont have previleges to use this command".to_string()));
    }
    let code  = Code::new(bnd.msg.content.clone())?;
    let pg = PgConn::create(bnd.init, bnd.msg.author.id.to_string()).await?;
    pg.execute(&code.code).await?;
    Components::msg(bnd, "success").await?;
    Ok(())
}
pub async fn msg_qry(bnd:&MsgBundle<'_>)->Result<(),MyErr>{
    if !bnd.msg.author.has_role(&bnd.ctx.http,bnd.msg.guild_id.unwrap().to_owned(), RoleId(NonZeroU64::new(bnd.init.server_role.admin_role).unwrap())).await?{
        return Err(MyErr::Custom("You dont have previleges to use this command".to_string()));
    }
    let code  = Code::new(bnd.msg.content.clone())?;
    let pg = PgConn::create(bnd.init, bnd.msg.author.id.to_string()).await?;
    let table = pg.query(&code.code).await?;
    Components::msg(bnd, &table).await?;
    Ok(())
}
