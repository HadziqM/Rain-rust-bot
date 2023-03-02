use std::time::Duration;
use serenity::all::*;
use serenity::futures::StreamExt;
use crate::{MyErr,SlashBundle,Mybundle,Mytrait,Reg,Components};
use crate::reusable::component::bounty::{Methode,BBQ,Category};


fn select_menu(id:&str,op:Vec<(String,String)>)->CreateSelectMenu{
    let options:Vec<_> = op.iter().map(|x|CreateSelectMenuOption::new(&x.0,&x.1)).collect();
    CreateSelectMenu::new(id, CreateSelectMenuKind::String {options}).min_values(1).min_values(1)
}
fn make_arow(menu:Vec<CreateSelectMenu>)->Vec<CreateActionRow>{
    menu.iter().map(|x|CreateActionRow::SelectMenu(x.to_owned())).collect()
}
async fn msg_edit(msg:&mut Message,arow:Vec<CreateActionRow>,ctx:&Context)->Result<(),MyErr>{
    msg.edit(&ctx.http, EditMessage::new().components(arow)).await?;
    Ok(())
}
#[hertz::hertz_slash_reg(300,false)]
async fn slash(bnd:&SlashBundle<'_>,reg:&Reg<'_>)->Result<(),MyErr>{
    let msg = bnd.cmd.data.resolved.messages.values().next()
        .ok_or(MyErr::Custom("cant get message".to_owned()))?;
    let link = &msg.attachments.first()
        .ok_or(MyErr::Custom("cant get your msg attachment".to_owned()))?.url;
    let mut m_menu = select_menu("method", Methode::option_str());
    let mut c_menu = select_menu("category", Category::option_str());
    let mut b_menu = select_menu("bbq", BBQ::option_str());
    bnd.cmd.create_response(&bnd.ctx.http, CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
            .components(make_arow(vec![m_menu.clone(),c_menu.clone(),b_menu.clone()])))).await?;
    let mut msg = bnd.cmd.get_msg(bnd.ctx).await?;
    let mut rply = msg.await_component_interactions(&bnd.ctx).timeout(Duration::new(30, 0)).stream();
    let mut methode = None;
    let mut category = None;
    let mut bbq = None;
    while let Some(pat) = rply.next().await {
        if pat.user != bnd.cmd.user{
            pat.response(bnd.ctx, Components::interaction_response("the menu isnt for you", true)).await?;
        }
        let wth = pat.data.custom_id.to_owned();
        match wth.as_str(){
            "method" => {
                if let ComponentInteractionDataKind::StringSelect { values } = &pat.data.kind{
                    methode = Some(values.first().unwrap().to_owned());
                    pat.defer(&bnd.ctx.http).await?;
                    if bbq.is_some() && category.is_some(){
                        break;
                    }
                    m_menu = m_menu.disabled(true);
                    msg_edit(&mut msg, make_arow(vec![m_menu.clone(),c_menu.clone(),b_menu.clone()]), bnd.ctx).await?;
                }
            }
            "category" => {
                if let ComponentInteractionDataKind::StringSelect { values } = &pat.data.kind{
                    category = Some(values.first().unwrap().to_owned());
                    pat.defer(&bnd.ctx.http).await?;
                    if bbq.is_some() && methode.is_some(){
                        break;
                    }
                    c_menu = c_menu.disabled(true);
                    msg_edit(&mut msg, make_arow(vec![m_menu.clone(),c_menu.clone(),b_menu.clone()]), bnd.ctx).await?;
                }
            }
            "bbq" => {
                if let ComponentInteractionDataKind::StringSelect { values } = &pat.data.kind{
                    bbq = Some(values.first().unwrap().to_owned());
                    pat.defer(&bnd.ctx.http).await?;
                    if methode.is_some() && category.is_some(){
                        break;
                    }
                    b_menu = b_menu.disabled(true);
                    msg_edit(&mut msg, make_arow(vec![m_menu.clone(),c_menu.clone(),b_menu.clone()]), bnd.ctx).await?;
                }
            }
            _ =>{continue;}
        }
    }
    Ok(())
}
