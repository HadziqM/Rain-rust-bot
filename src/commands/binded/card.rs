use crate::{Reg,SlashBundle,MyErr,Mytrait,Mybundle};

#[hertz::hertz_slash_reg(10,false)]
async fn slash(bnd:&SlashBundle<'_>,mut reg:Reg<'_>)->Result<(),MyErr>{
    let card =  reg.pg.get_card(reg.cid).await?;
    bnd.cmd.create_response(&bnd.ctx.http,card.card(&bnd.cmd.user,false)).await?;
    reg.pg.close().await;
    Ok(())
}

#[hertz::hertz_slash_normal(0,false)]
async fn slash_user(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
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
