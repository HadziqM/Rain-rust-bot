use crate::{MyErr,Mybundle,Mytrait,Components,SlashBundle,Reg,PgConn};
use crate::reusable::component::bounty::{Bounty,BountyRefresh,BBQ};

#[hertz::hertz_slash_normal(0,false)]
async fn slash(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    let mut name = "";
    for sub in &bnd.cmd.data.options{
        if let serenity::all::CommandDataOptionValue::SubCommand(_) = &sub.value{
            name = sub.name.as_str();
        }
    }
    match name{
        "refresh"=>refresh(bnd).await?,
        "user"=>reset(bnd).await?,
        "bounty"=>bounty(bnd).await?,
        _ => {return Err(MyErr::Custom("cant get the correct sub name".to_owned()))}
    };
    Ok(())
}

fn unsafe_allocate_bounty() -> Result<Box<Bounty>,MyErr> {
    let grid_box: Box<Bounty>;
    println!("unsafe block");
    unsafe {
        use std::alloc::{alloc, Layout};
        let layout = Layout::new::<Bounty>();
        let ptr = alloc(layout) as *mut Bounty;
        println!("allocated");
        (*ptr) = serde_json::from_slice(&std::fs::read(Bounty::path()).unwrap())?;
        println!("populate pointer");
        grid_box = Box::from_raw(ptr);
    }
    return Ok(grid_box);
}
async fn refresh(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    println!("try to load bounty_refresh.json");
    let refresh =BountyRefresh::new().await?;
    println!("try to load bounty.json");
    let mut bounty = unsafe_allocate_bounty()?;
    println!("try to change serialized bounty");
    bounty.refresh(&refresh);
    bounty.save().await?;
    Components::response(bnd, "success", true).await?;
    bounty.cooldown(bnd).await?;
    Ok(())
}
async fn bounty(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    let mut bb = String::new();
    let mut cd = 0;
    for i in Components::sub_options(bnd)?{
        match &i.value{
            serenity::all::CommandDataOptionValue::Integer(x)=>{cd=*x}
            serenity::all::CommandDataOptionValue::String(x)=>{bb=x.to_owned()}
            _ =>{continue;}
        }
    }
    let bbq = BBQ::new(bb.parse::<u8>().unwrap())?;
    let mut bounty = Bounty::new().await?;
    bounty.set_cd(&bbq, cd as u32);
    bounty.save().await?;
    Components::response(bnd, "success", true).await?;
    bounty.cooldown(bnd).await?;
    Ok(())
}
async fn reset(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    let user = bnd.cmd.data.resolved.users.values().next()
        .ok_or(MyErr::Custom("cant get user".to_owned()))?;
    let mut reg = Reg::check(bnd, user).await?;
    reg.pg.reset().await?;
    Components::response(bnd, &format!("{} bounty cooldown resetted",user.to_string()), false).await?;
    reg.pg.close().await;
    Ok(())
}

use crate::reusable::bitwise::BitwiseError;

impl PgConn<'_>{
    async fn reset(&self)->Result<(),BitwiseError>{
        sqlx::query("update discord set latest_bounty_time=0 where discord_id=$1")
            .bind(&self.did).execute(&self.pool).await?;
        Ok(())
    }
}


