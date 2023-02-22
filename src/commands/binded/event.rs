use crate::{Reg,SlashBundle,MyErr,Mytrait,Mybundle};

#[hertz::hertz_slash_reg(10,false)]
async fn slash(bnd:&SlashBundle<'_>,reg:&Reg<'_>)->Result<(),MyErr>{
    let event =  reg.pg.get_event().await?;
    let user = bnd.user();
    event.response(&user, bnd).await?;
    Ok(())
}

#[hertz::hertz_slash_normal(0,false)]
async fn userr(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    let user =match bnd.cmd.data.resolved.users.iter().next(){
        Some((_id,u))=>u,
        None=>{return Err(MyErr::Custom("no user detected".to_string()));}
    };
    let mut reg =Reg::check(bnd, &user).await?;
    let event = match reg.pg.get_event().await{
        Ok(x)=>x,
        Err(y)=>{
            reg.pg.close().await;
            return Err(y.into());
        }
    };
    if let Err(y)=event.response(user, bnd).await{
        reg.pg.close().await;
        return Err(y);
    }
    reg.pg.close().await;
    Ok(())
}
