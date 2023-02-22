use serenity::all::*;
use crate::{commands, COOLDOWN};
use crate::{Init,ItemPedia,MyErr,ErrorLog,Mytrait,Reg,Images};
use crate::reusable::utils::MyTime;
use serenity::async_trait;

pub struct SlashBundle<'a>{
    pub cmd:&'a CommandInteraction,
    pub ctx:&'a Context,
    pub init:&'a Init,
    pub pedia:&'a ItemPedia,
    pub image:&'a Images
}
pub struct ComponentBundle<'a>{
    pub cmd:&'a ComponentInteraction,
    pub ctx:&'a Context,
    pub init:&'a Init,
    pub pedia:&'a ItemPedia,
    pub image:&'a Images
}
pub struct ModalBundle<'a>{
    pub cmd:&'a ModalInteraction,
    pub ctx:&'a Context,
    pub init:&'a Init,
    pub pedia:&'a ItemPedia,
    pub image:&'a Images
}

#[async_trait]
pub trait Mybundle {
    type Cmd:Mytrait;
    fn ctx<'a>(&'a self)->&'a Context;
    fn init<'a>(&'a self)->&'a Init;
    fn user(&self)->User;
    fn cmd<'a>(&'a self)->&'a Self::Cmd;
    fn name(&self)->String;
    async fn cooldown(&self,cd:i64)->Result<(),MyErr>;
}


#[async_trait]
impl Mybundle for SlashBundle<'_>{
    type Cmd = CommandInteraction;
    fn ctx<'a>(&'a self)->&'a Context {
        self.ctx
    }
    fn init<'a>(&'a self)->&'a Init {
        self.init
    }
    fn user(&self)->User {
        self.cmd.user.clone()
    }
    fn cmd<'a>(&'a self)->&'a Self::Cmd {
        self.cmd
    }
    fn name(&self)->String {
        self.cmd.data.name.clone()
    }
    async fn cooldown(&self,time:i64)->Result<(),MyErr> {
        if time==0{
            return Ok(());
        }
        let pat = format!("{}-{}",self.name(),self.user().id.to_string());
        let mut cd = COOLDOWN.lock().await;
        let now = MyTime::now();
        match cd.get_mut(&pat){
            Some(x)=>{
                if *x as i64 > now{
                    return Err(MyErr::Custom(format!("youare still on cooldown to use this command wait till <t:{}:R>",x)));
                }else{
                    *x = now + time;
                    return Ok(());
                }
            }
            None=>{
                cd.insert(pat.to_owned(), now+time);
                return Ok(());
            }
        }
    }
}
#[async_trait]
impl Mybundle for ComponentBundle<'_>{
    type Cmd = ComponentInteraction;
        fn ctx<'a>(&'a self)->&'a Context {
        self.ctx
    }
    fn init<'a>(&'a self)->&'a Init {
        self.init
    }
    fn user(&self)->User {
        self.cmd.user.clone()
    }
    fn cmd<'a>(&'a self)->&'a Self::Cmd {
        self.cmd
    }
    fn name(&self)->String {
        self.cmd.data.custom_id.clone()
    }
    async fn cooldown(&self,time:i64)->Result<(),MyErr> {
        if time==0{
            return Ok(());
        }
        let pat = format!("{}-{}",self.name(),self.user().id.to_string());
        let mut cd = COOLDOWN.lock().await;
        let now = MyTime::now();
        match cd.get_mut(&pat){
            Some(x)=>{
                if *x as i64 > now{
                    return Err(MyErr::Custom(format!("youare still on cooldown to use this command wait till <t:{}:R>",x)));
                }else{
                    *x = now + time;
                    return Ok(());
                }
            }
            None=>{
                cd.insert(pat.to_owned(), now+time);
                return Ok(());
            }
        }
    }
}
#[async_trait]
impl Mybundle for ModalBundle<'_>{
    type Cmd = ModalInteraction;
    fn ctx<'a>(&'a self)->&'a Context {
        self.ctx
    }
    fn init<'a>(&'a self)->&'a Init {
        self.init
    }
    fn user(&self)->User {
        self.cmd.user.clone()
    }
    fn cmd<'a>(&'a self)->&'a Self::Cmd {
        self.cmd
    }
    fn name(&self)->String {
        self.cmd.data.custom_id.clone()
    }
    async fn cooldown(&self,time:i64)->Result<(),MyErr> {
        if time==0{
            return Ok(());
        }
        let pat = format!("{}-{}",self.name(),self.user().id.to_string());
        let mut cd = COOLDOWN.lock().await;
        let now = MyTime::now();
        match cd.get_mut(&pat){
            Some(x)=>{
                if *x as i64 > now{
                    return Err(MyErr::Custom(format!("youare still on cooldown to use this command wait till <t:{}:R>",x)));
                }else{
                    *x = now + time;
                    return Ok(());
                }
            }
            None=>{
                cd.insert(pat.to_owned(), now+time);
                return Ok(());
            }
        }
    }
}
// async fn normal<'a,F:Fn(&'a T)->Fut,Fut:Future<Output = Result<(),MyErr>>,T:Mybundle>(bnd:&'a T,cd:i64,on:&'static str,ephemeral:bool,defer:bool,f:F){
//     let user = bnd.user();
//     let cmd = bnd.cmd();
//     let mut err = ErrorLog::new(bnd.ctx(),bnd.init(),&user).await;
//     if cd != 0{
//         let cooldown = bnd.cooldown(cd).await;
//         if !cooldown.0{
//             let er = MyErr::Custom(format!("youare still on cooldown to use this command wait till <t:{}:R>",cooldown.1));
//             return er.log(cmd, on, ephemeral, &mut err).await;
//         }
//     }
//     if defer{
//         cmd.defer_res(&mut err, on,ephemeral).await;
//     }
//     if let Err(why)=f(bnd).await{
//         match defer{
//             true=>why.log(cmd, on, ephemeral, &mut err).await,
//             false=>why.log_defer(cmd, on, &mut err).await,
//         };
//     }
// }
// async fn register<'a,F,Fut,T>(bnd:&'a T,cd:i64,on:&'static str,ephemeral:bool,defer:bool,bypass:bool,f:F) where F:Fn(&'a T,Reg<'a>)->Fut,Fut:Future<Output = Result<(),MyErr>>,T:Mybundle{
//     let user = bnd.user();
//     let cmd = bnd.cmd();
//     let mut err = ErrorLog::new(bnd.ctx(),bnd.init(),&user).await;
//     if cd != 0{
//         let cooldown = bnd.cooldown(cd).await;
//         if !cooldown.0{
//             let er = MyErr::Custom(format!("youare still on cooldown to use this command wait till <t:{}:R>",cooldown.1));
//             return er.log(cmd, on, ephemeral, &mut err).await;
//         }
//     }
//     let reg = match Reg::switch(bnd.ctx(), cmd, bnd.init(), bypass, ephemeral).await{
//         Ok(x)=>{
//             match x{
//                 Some(y)=>y,
//                 None=>{return;}
//             }
//         }
//         Err(why)=>{
//             return why.log(cmd, on, ephemeral, &mut err).await;
//         }
//     };
//     if defer{
//         cmd.defer_res(&mut err, on, ephemeral).await;
//     }
//     if let Err(why)=f(bnd,reg).await{
//         match defer{
//             true=>why.log(cmd, on, ephemeral, &mut err).await,
//             false=>why.log_defer(cmd, on, &mut err).await,
//         };
//     }
// }
pub async fn handled(ctx:&Context,int:&Interaction,pedia:&ItemPedia,init:&Init,image:&Images){
    match int{
        Interaction::Command(cmd)=>{
            let bnd = SlashBundle{cmd,init,image,pedia,ctx};
            let wth = cmd.data.name.as_str();
            match wth{
                "interface"=>commands::admin::interface::discord_slash(&bnd).await,
                "create"=>commands::register::create::discord_all(&bnd).await,
                "bind"=>commands::register::create::discord_all(&bnd).await,
                "change_password"=>commands::register::change_pasword::discord_slash(&bnd).await,
                "card"=>commands::binded::card::discord_slash(&bnd).await,
                "event"=>commands::binded::event::discord_slash(&bnd).await,
                "switch"=>switch(&bnd).await,
                "👤 Card"=>commands::binded::card::discord_userr(&bnd).await,
                "🎀 Event"=>commands::binded::event::discord_userr(&bnd).await,
                "dm_save"=>commands::binded::save::discord_all(&bnd).await,
                "transfer"=>commands::binded::transfer::discord_slash(&bnd).await,
                "market"=>commands::admin::market::discord_slash(&bnd).await,
                "purge"=>commands::admin::purge::discord_slash(&bnd).await,
                "pull"=>commands::gacha::pull::discord_slash(&bnd).await,
                "config"=>commands::admin::config::discord_slash(&bnd).await,
                "stall"=>commands::market::market::discord_slash(&bnd).await,
                "check"=>commands::register::change_pasword::discord_check(&bnd).await,
                "ferias"=>commands::misc::ferias::discord_slash(&bnd).await,
                "monitor"=>commands::admin::monitor::discord_auto(&bnd).await,
                _=> {return;}
            }
        }
        Interaction::Component(cmd)=>{
            let bnd = ComponentBundle{cmd,init,image,pedia,ctx};
            let wth = cmd.data.custom_id.as_str();
            if wth.contains("save"){
                return commands::binded::transfer::discord_button(&bnd).await;
            }
            match wth{
                "register"=>commands::register::create::discord_all(&bnd).await,
                "bind"=>commands::register::create::discord_all(&bnd).await,
                "dms"=>commands::binded::save::discord_all(&bnd).await,
                "switch"=>switch(&bnd).await,
                _=>{return ;}
            }
        }
        Interaction::Modal(cmd)=>{
            let bnd = ModalBundle{cmd,init,image,pedia,ctx};
            let wth = cmd.data.custom_id.as_str();
            match wth{
                "register_m"=>commands::register::create::discord_modal(&bnd).await,
                "bind"=>commands::register::create::discord_modal(&bnd).await,
                _=>{return;}
            }
        }
        Interaction::Autocomplete(cmd)=>{
            let bnd = SlashBundle{cmd,init,image,pedia,ctx};
            let wth = cmd.data.name.as_str();
            match wth{
                "market"=>commands::admin::market::discord_auto(&bnd).await,
                "stall"=>commands::market::market::discord_auto(&bnd).await,
                "ferias"=>commands::misc::ferias::discord_auto(&bnd).await,
                _=>{return;}
            }
        }
        _=>{return ;}
    }
}
async fn switch<T:Mybundle>(bnd:&T){
    let on  = bnd.name();
    let user = bnd.user();
    let cmd = bnd.cmd();
    let mut err = ErrorLog::new(bnd.ctx(),bnd.init(),&user).await;
    if let Err(why)=bnd.cooldown(60).await{
        return why.log(cmd, &on, true, &mut err).await;
    }
    match Reg::switch(bnd.ctx(), cmd, bnd.init(), true,true).await{
        Ok(x)=>{
            match x{
                Some(y)=>y,
                None=>{return;}
            }
        }
        Err(why)=>{
            return why.log(cmd, &on,true, &mut err).await;
        }
    };
}
