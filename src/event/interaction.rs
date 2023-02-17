use serenity::all::*;
use serenity::futures::Future;
use crate::{commands, COOLDOWN};
use crate::{Init,ItemPedia,MyErr,ErrorLog,Mytrait,Reg,Images};
use crate::reusable::utils::MyTime;

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

pub trait Mybundle {
    type Cmd:Mytrait;
    fn ctx<'a>(&'a self)->&'a Context;
    fn init<'a>(&'a self)->&'a Init;
    fn user(&self)->User;
    fn cmd<'a>(&'a self)->&'a Self::Cmd;
    fn name(&self)->String;
    fn cooldown(&self,cd:i64)->(bool,i64);
}
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
    fn cooldown(&self,time:i64)->(bool,i64) {
        let pat = format!("{}-{}",self.name(),self.user().id.to_string());
        let mut cd = COOLDOWN.lock().unwrap();
        let now = MyTime::now();
        match cd.get_mut(&pat){
            Some(x)=>{
                let y = x.clone();
                if *x as i64 > now{
                    *x = now + time;
                    return (false,y);
                }else{
                    *x = now + time;
                    return (true,y);
                }
            }
            None=>{
                cd.insert(pat.to_owned(), now+time);
                return (true,0);
            }
        }
    }
}
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
    fn cooldown(&self,time:i64)->(bool,i64) {
        let pat = format!("{}-{}",self.name(),self.user().id.to_string());
        let mut cd = COOLDOWN.lock().unwrap();
        let now = MyTime::now();
        match cd.get_mut(&pat){
            Some(x)=>{
                let y = x.clone();
                if *x as i64 > now{
                    *x = now + time;
                    return (false,y);
                }else{
                    *x = now + time;
                    return (true,y);
                }
            }
            None=>{
                cd.insert(pat.to_owned(), now+time);
                return (true,0);
            }
        }
    }
}
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
    fn cooldown(&self,time:i64)->(bool,i64) {
        let pat = format!("{}-{}",self.name(),self.user().id.to_string());
        let mut cd = COOLDOWN.lock().unwrap();
        let now = MyTime::now();
        match cd.get_mut(&pat){
            Some(x)=>{
                let y = x.clone();
                if *x as i64 > now{
                    *x = now + time;
                    return (false,y);
                }else{
                    *x = now + time;
                    return (true,y);
                }
            }
            None=>{
                cd.insert(pat.to_owned(), now+time);
                return (true,0);
            }
        }
    }
}
async fn normal<'a,F:Fn(&'a T)->Fut,Fut:Future<Output = Result<(),MyErr>>,T:Mybundle>(bnd:&'a T,cd:i64,on:&'static str,ephemeral:bool,defer:bool,f:F){
    let user = bnd.user();
    let cmd = bnd.cmd();
    let mut err = ErrorLog::new(bnd.ctx(),bnd.init(),&user).await;
    if cd != 0{
        let cooldown = bnd.cooldown(cd);
        if !cooldown.0{
            let er = MyErr::Custom(format!("youare still on cooldown to use this command wait till <t:{}:R>",cooldown.1));
            return er.log(cmd, on, ephemeral, &mut err).await;
        }
    }
    if defer{
        cmd.defer_res(&mut err, on,ephemeral).await;
    }
    if let Err(why)=f(bnd).await{
        match defer{
            true=>why.log(cmd, on, ephemeral, &mut err).await,
            false=>why.log_defer(cmd, on, &mut err).await,
        };
    }
}
async fn register<'a,F,Fut,T>(bnd:&'a T,cd:i64,on:&'static str,ephemeral:bool,defer:bool,bypass:bool,f:F) where F:Fn(&'a T,Reg<'a>)->Fut,Fut:Future<Output = Result<(),MyErr>>,T:Mybundle{
    let user = bnd.user();
    let cmd = bnd.cmd();
    let mut err = ErrorLog::new(bnd.ctx(),bnd.init(),&user).await;
    if cd != 0{
        let cooldown = bnd.cooldown(cd);
        if !cooldown.0{
            let er = MyErr::Custom(format!("youare still on cooldown to use this command wait till <t:{}:R>",cooldown.1));
            return er.log(cmd, on, ephemeral, &mut err).await;
        }
    }
    let reg = match Reg::switch(bnd.ctx(), cmd, bnd.init(), bypass, ephemeral).await{
        Ok(x)=>{
            match x{
                Some(y)=>y,
                None=>{return;}
            }
        }
        Err(why)=>{
            return why.log(cmd, on, ephemeral, &mut err).await;
        }
    };
    if defer{
        cmd.defer_res(&mut err, on, ephemeral).await;
    }
    if let Err(why)=f(bnd,reg).await{
        match defer{
            true=>why.log(cmd, on, ephemeral, &mut err).await,
            false=>why.log_defer(cmd, on, &mut err).await,
        };
    }
}
pub async fn handled(ctx:&Context,int:&Interaction,pedia:&ItemPedia,init:&Init,image:&Images){
    match int{
        Interaction::Command(cmd)=>{
            let bnd = SlashBundle{cmd,init,image,pedia,ctx};
            let wth = cmd.data.name.as_str();
            match wth{
                "interface"=>normal(&bnd, 60, "interface", false, false, commands::admin::interface::slash).await,
                "create"=>normal(&bnd, 60, "register", false, false, commands::register::create::all).await,
                "bind"=>normal(&bnd, 60, "bind", false, false, commands::register::create::all).await,
                "change_password"=>register(&bnd,10*60, "change_password", true, false, false, commands::register::change_pasword::slash).await,
                "card"=>register(&bnd, 60, "card", false, false, false, commands::binded::card::slash).await,
                "switch"=>register(&bnd, 60, "switch", false, false, true, dummy).await,
                "👤 Card"=>normal(&bnd, 10, "card Context", false, false, commands::binded::card::slash_user).await,
                "dm_save"=>register(&bnd, 60, "dm user save file", true, false, false, commands::binded::save::all).await,
                "transfer"=>register(&bnd, 5*60, "transfer command", false, true, false, commands::binded::transfer::slash).await,
                "market"=>normal(&bnd, 10, "admin market", false, false, commands::admin::market::slash).await,
                "purge"=>normal(&bnd, 10, "purge", true, false, commands::admin::purge::slash).await,
                "pull"=>register(&bnd, 10, "pull gacha", false, true, false, commands::gacha::pull::slash).await,
                "config"=>normal(&bnd, 30, "config", true, false, commands::admin::config::slash).await,
                "stall"=>register(&bnd, 60, "stall", false, false, false, commands::market::market::slash).await,
                _=> {return;}
            }
        }
        Interaction::Component(cmd)=>{
            let bnd = ComponentBundle{cmd,init,image,pedia,ctx};
            let wth = cmd.data.custom_id.as_str();
            if wth.contains("save"){
                return normal(&bnd, 0, "save acknowledge", true, false, commands::binded::transfer::button).await;
            }
            match wth{
                "register"=>normal(&bnd, 40, "register", true, false, commands::register::create::all).await,
                "bind"=>normal(&bnd, 40, "bind", true, false, commands::register::create::all).await,
                "dms"=>register(&bnd, 60, "dm user save", true, false,false, commands::binded::save::all).await,
                _=>{return ;}
            }
        }
        Interaction::Modal(cmd)=>{
            let bnd = ModalBundle{cmd,init,image,pedia,ctx};
            let wth = cmd.data.custom_id.as_str();
            match wth{
                "register_m"=>normal(&bnd, 0, "modal register", true, false, commands::register::create::all).await,
                "bind"=>normal(&bnd, 0, "modal register", true, false, commands::register::create::all).await,
                _=>{return;}
            }
        }
        Interaction::Autocomplete(cmd)=>{
            let bnd = SlashBundle{cmd,init,image,pedia,ctx};
            let wth = cmd.data.name.as_str();
            match wth{
                "market"=>normal(&bnd, 0, "market Autocomplete", true, false, commands::admin::market::auto).await,
                "stall"=>normal(&bnd, 0, "stall Autocomplete",false, false, commands::market::market::auto).await,
                _=>{return;}
            }
        }
        _=>{return ;}
    }
}
async fn dummy<T:Mybundle>(_bnd:&T,_reg:Reg<'_>)->Result<(),MyErr>{
    Ok(())
}
