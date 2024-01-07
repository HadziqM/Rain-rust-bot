use std::num::NonZeroU64;
use crate::reusable::utils::color;
use crate::{MyErr,ModalBundle,Reg,Components,Mybundle,Mytrait};
use serenity::all::*;

struct RegisterAcknowledged<'a,'b> {
    name: &'a str,
    uid:i32,
    bnd:&'a ModalBundle<'b>
}

impl<'a,'b> RegisterAcknowledged<'a,'b>{
    fn new(name:&'a str,uid:i32,bnd:&'a ModalBundle<'b>)->RegisterAcknowledged<'a,'b>{
        RegisterAcknowledged { name, uid, bnd }
    }
    async fn add_roles(&self)->Result<(),MyErr>{
        let rid = RoleId(NonZeroU64::new(self.bnd.init.server_role.register_role).unwrap());
        let mut user = self.bnd.cmd.member.clone().unwrap();
        user.add_role(&self.bnd.ctx.http,rid).await?;
        Ok(())
    }
    async fn log_to_user(&self,reg:bool)->Result<(),MyErr>{
        let word = || {if reg{return "Created";}"binded"};
        let user = self.bnd.user();
        let ch = ChannelId(NonZeroU64::new(self.bnd.init.log_channel.account_channel).unwrap());
        ch.send_message(&self.bnd.ctx.http,CreateMessage::new()
            .content(format!("{}",user.to_string()))
            .embed(CreateEmbed::new().title(format!("Account Succesfully {} on Server",word()))
                .description(&format!("{} {} an account on server, remember that you still need to have a character in game to fully use our discord features so hurry up and create one if you havent",user.to_string(),word())).fields(vec![
                    ("👤 Username",&format!("`{}`",self.name),false),
                    ("🆔 User Id",&format!("`{}`",self.uid),false)
                ]).author(CreateEmbedAuthor::new(&user.name).icon_url(user.face()))
            .colour(color("00", "ff", "00"))
            .image("https://media.discordapp.net/attachments/1068440173479739393/1068458599627620392/cachedImage.png?width=807&height=455"))).await?;
        Ok(())
    }
}
fn modal_register_row(name:&str,pass:bool)->CreateActionRow{
    let placeholder = match pass {
        false => "your MHFZ username on launcher".to_owned(),
        true => "your MHFZ user password (igonore discord warning)".to_owned(),
    };
    CreateActionRow::InputText(
        CreateInputText::new(InputTextStyle::Short, name, name).required(true).placeholder(&placeholder)
    )
}

fn modal_response(reg:bool)->CreateInteractionResponse{
    let name;
    let title;
    if reg{
        name="register_m";
        title="Register Command";
    }else{
        name="bind_m";
        title="Bind Command";
    }
    CreateInteractionResponse::Modal(CreateModal::new(name, title)
        .components(vec![modal_register_row("username", false),
            modal_register_row("password", true),
            CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Short, "PSN_ID", "psn_id")
                .required(false).placeholder("For console player (You can Leave it Empty)"))]))
}

#[hertz::hertz_combine_normal(60,false)]
async fn all<T:Mybundle>(bnd:&T)->Result<(),MyErr>{
    let regist = match bnd.name().as_str(){
        "bind"=>false,
        _=>true
    };
    let user = bnd.user();
    if let Some(mut reg) = Reg::reverse_check(bnd,&user).await?{
        Components::response_adv(bnd, modal_response(regist)).await?;
        reg.pg.close().await;
    };
    Ok(())
}

#[hertz::hertz_modal_normal(0,false)]
async fn modal(bnd:&ModalBundle<'_>)->Result<(),MyErr>{
    let regist = match bnd.name().as_str(){
        "bind_m"=>false,
        _=>true
    };
    let user = bnd.user();
    let reg = Reg::no_check(bnd, &user).await?;
    let mut name = String::new();
    let mut password = String::new();
    let mut psn = String::new();
    for comp in &bnd.cmd.data.components{
        let arow = comp.components.first().unwrap();
        if let ActionRowComponent::InputText(input) = arow{
            match input.custom_id.as_str() {
                "username" => name = input.value.to_owned(),
                "password"=> password = input.value.to_owned(),
                _ => psn = input.value.to_owned(),
            }
        }
    }
    let uid = reg.pg.create_account(&name, &password, &psn, regist).await?;
    let resp = RegisterAcknowledged::new(&name, uid.id, bnd);
    resp.log_to_user(regist).await?;
    resp.add_roles().await?;
    Components::response(bnd, "account succesfully Created", true).await?;
    Ok(())
}