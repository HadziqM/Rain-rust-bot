use crate::{Reg,SlashBundle,MyErr};

pub async fn slash(bnd:&SlashBundle<'_>,mut reg:Reg<'_>)->Result<(),MyErr>{
    let card =  reg.pg.get_card(reg.cid).await?;
    bnd.cmd.create_response(&bnd.ctx.http,card.card(&bnd.cmd.user,false)).await?;
    reg.pg.close().await;
    Ok(())
}
pub async fn slash_user(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    let user =match bnd.cmd.data.resolved.users.iter().next(){
        Some((_id,u))=>u,
        None=>{return Err(MyErr::Custom("no user detected".to_string()));}
    };
    let mut reg = Reg::check(bnd, &user).await?;
    let card = reg.pg.get_card(reg.cid).await?;
    bnd.cmd.create_response(&bnd.ctx.http,card.card(&user,false)).await?;
    reg.pg.close().await;
    Ok(())
}
