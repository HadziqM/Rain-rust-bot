use serenity::all::*;
use crate::{MyErr,Mytrait,Mybundle,SlashBundle,Components,PgConn};


#[hertz::hertz_slash_normal(0,false)]
async fn slash(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    let mut bounty = false;
    let mut all = false;
    let mut amount = 0;
    let mut user = bnd.cmd.user.id.to_string();
    for data in &bnd.cmd.data.options{
        if let CommandDataOptionValue::SubCommandGroup(x) = &data.value{
            if data.name.as_str() == "bounty"{
                bounty = true;
            }
            for data2 in x{
                if let CommandDataOptionValue::SubCommand(y) = &data2.value{
                    if data2.name.as_str() == "all"{
                        all = true;
                    }
                    for data3 in y{
                        match data3.value{
                            CommandDataOptionValue::Integer(i)=>{
                                amount = i;
                            }
                            CommandDataOptionValue::User(u)=>{
                                user = u.to_string()
                            }
                            _ =>{continue;}
                        }
                    }
                }
            }
        }
    }
    let mut pg = PgConn::create(bnd.init, user).await?;
    if bounty{
        if all{
            pg.bounty_all(amount as i32).await?;
        }else {
            pg.bounty_transaction(-1 * amount as i32).await?;
        }
    }else {
        if all{
            pg.ticket_all(amount as i32).await?;
        }else {
            pg.buy_ticket(amount as i32).await?;
        }
    }
    pg.close().await;
    Components::response(bnd, "Succeeded", true).await?;
    Ok(())
}
