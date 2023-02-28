use crate::SlashBundle;
use super::{Components,MyErr};
use serenity::all::*;
use super::bounty::{Bounty,BountyRefresh,BountyTitle};
use serenity::async_trait;
use super::gacha::Gacha;
use super::market::{Market,Meal,Trading,Tag};


impl Components {
    pub fn get_att(cmd:&CommandInteraction)->Result<Attachment,MyErr>{
        let resolved:Vec<_>=cmd.data.resolved.attachments.iter().map(|x|x.1).collect();
        match resolved.first(){
            Some(x)=>{
                let idk = *x;
                Ok(idk.clone())
            }
            None=>Err(MyErr::Custom("cant get the attachment attachment".to_owned()))
        }
    }
    pub async fn json_config<T:MyConfig>(bnd:&SlashBundle<'_>,_tip:T)->Result<(),MyErr>{
        let att = Components::get_att(bnd.cmd)?;
        let byte = att.download().await?;
        let utf8 = std::str::from_utf8(&byte)?.to_owned();
        T::check(&utf8).await?;
        T::update(bnd).await?;
        Ok(())
    }
}
#[async_trait]
pub trait MyConfig {
    async fn check(data:&str)->Result<(),MyErr>;
    async fn update(bnd:&SlashBundle<'_>)->Result<(),MyErr>;
}


#[async_trait]
impl MyConfig for Bounty{
    async fn check(data:&str)->Result<(),MyErr>{
        Ok(Bounty::check(data).await?)
    }
    async fn update(_bnd:&SlashBundle<'_>)->Result<(),MyErr>{
        Ok(())
    }
}
#[async_trait]
impl MyConfig for BountyRefresh {
    async fn check(data:&str)->Result<(),MyErr>{
        Ok(BountyRefresh::check(data).await?)
    }
    async fn update(_bnd:&SlashBundle<'_>)->Result<(),MyErr>{
        Ok(())
    }
}
#[async_trait]
impl MyConfig for BountyTitle {
    async fn check(data:&str)->Result<(),MyErr>{
        Ok(BountyTitle::check(data).await?)
    }
    async fn update(_bnd:&SlashBundle<'_>)->Result<(),MyErr>{
        Ok(())
    }
}
#[async_trait]
impl MyConfig for Gacha {
    async fn check(data:&str)->Result<(),MyErr>{
        Ok(Gacha::check(data).await?)
    }
    async fn update(_bnd:&SlashBundle<'_>)->Result<(),MyErr>{
        Ok(())
    }
}
#[async_trait]
impl MyConfig for Market {
    async fn check(data:&str)->Result<(),MyErr>{
        Ok(Market::check(data).await?)
    }
    async fn update(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
        Market::update_new(bnd.ctx, bnd.init, bnd.pedia).await?;
        Ok(())
    }
}
#[async_trait]
impl MyConfig for Meal {
    async fn check(data:&str)->Result<(),MyErr>{
        Ok(Meal::check(data).await?)
    }
    async fn update(_bnd:&SlashBundle<'_>)->Result<(),MyErr>{
        Ok(())
    }
}
#[async_trait]
impl MyConfig for Trading {
    async fn check(data:&str)->Result<(),MyErr>{
        Ok(Trading::check(data).await?)
    }
    async fn update(_bnd:&SlashBundle<'_>)->Result<(),MyErr>{
        Ok(())
    }
}
#[async_trait]
impl MyConfig for Tag {
    async fn check(data:&str)->Result<(),MyErr>{
        Ok(Tag::check(data).await?)
    }
    async fn update(_bnd:&SlashBundle<'_>)->Result<(),MyErr>{
        Ok(())
    }
}
