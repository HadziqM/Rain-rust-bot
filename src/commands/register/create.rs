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
        let server = self.bnd.cmd.guild_id.unwrap_or_default().to_partial_guild(&self.bnd.ctx.http).await?;
        let ch = ChannelId(NonZeroU64::new(self.bnd.init.log_channel.account_channel).unwrap());
        ch.send_message(&self.bnd.ctx.http,CreateMessage::new().embed(CreateEmbed::new().title(format!("Account Succesfully {} on Server",word()))
                .description(&format!("{} {} an account on server, by creating account here you already aggree to follow our rules to as stated on rules channel, as a member of {} comunity we welcome you to enjoy the game together",user.to_string(),word(),server.name)).fields(vec![
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
        .components(vec![modal_register_row("username", false),modal_register_row("password", true)]))
}

#[hertz::hertz_combine_normal(60,false)]
async fn all<T:Mybundle>(bnd:&T)->Result<(),MyErr>{
    let regist = match bnd.name().as_str(){
        "bind"=>false,
        _=>true
    };
    let user = bnd.user();
    let mut reg = Reg::reverse_check(bnd,&user).await?;
    Components::response_adv(bnd, modal_response(regist)).await?;
    reg.pg.close().await;
    Ok(())
}

#[hertz::hertz_modal_normal(0,false)]
async fn modal(bnd:&ModalBundle<'_>)->Result<(),MyErr>{
    let regist = match bnd.name().as_str(){
        "bind_m"=>false,
        _=>true
    };
    let user = bnd.user();
    let reg = Reg::reverse_check(bnd, &user).await?;
    let mut name = String::new();
    let mut password = String::new();
    for comp in &bnd.cmd.data.components{
        let arow = comp.components.first().unwrap();
        if let ActionRowComponent::InputText(input) = arow{
            match input.custom_id.as_str() {
                "username" => name = input.value.to_owned(),
                 _=> password = input.value.to_owned(),
            }
        }
    }
    let uid = reg.pg.create_account(&name, &password, regist).await?;
    let resp = RegisterAcknowledged::new(&name, uid.id, bnd);
    resp.log_to_user(regist).await?;
    resp.add_roles().await?;
    Components::response(bnd, "account succesfully Created", true).await?;
    Ok(())
}
