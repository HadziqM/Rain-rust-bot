use std::time::Duration;
use serenity::all::*;
use serenity::futures::StreamExt;
use crate::{MyErr,SlashBundle,Mybundle,Mytrait,PgConn,Components,ComponentBundle};
use crate::reusable::component::bounty::{Methode,BBQ,Category,BountySubmit,Bounty,BountyTitle};


fn select_menu(id:&str,op:Vec<(String,String)>)->CreateSelectMenu{
    let options:Vec<_> = op.iter().map(|x|CreateSelectMenuOption::new(&x.1,&x.0)).collect();
    CreateSelectMenu::new(id, CreateSelectMenuKind::String {options}).min_values(1).min_values(1)
}
fn make_arow(menu:Vec<CreateSelectMenu>)->Vec<CreateActionRow>{
    menu.iter().map(|x|CreateActionRow::SelectMenu(x.to_owned())).collect()
}
async fn msg_edit(msg:&mut Message,arow:Vec<CreateActionRow>,ctx:&Context)->Result<(),MyErr>{
    msg.edit(&ctx.http, EditMessage::new().components(arow)).await?;
    Ok(())
}
fn mentions(ment:&str)->Result<Vec<UserId>,MyErr>{
    let mut out = Vec::new();
    for i in ment.split(">"){
        let val;
        if i.contains("<!@"){
            val = i.replace("<!@","").trim().to_owned();
        }else {
            val = i.replace("<@","").trim().to_owned();
        }
        let id = val.parse::<u64>().ok()
            .ok_or(MyErr::Custom("my dumb regex failed to get user".to_owned()))?;
        out.push(UserId::new(id))
    }
    Ok(out)
}

#[hertz::hertz_slash_normal(300,false)]
async fn slash(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
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
                    methode = Some(values.first().unwrap().to_owned().parse::<u8>().unwrap());
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
                    category = Some(values.first().unwrap().to_owned().parse::<u8>().unwrap());
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
                    bbq = Some(values.first().unwrap().to_owned().parse::<u8>().unwrap());
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
    let mem = *bnd.cmd.member.clone().unwrap();
    let mut pg = PgConn::create(bnd.init,bnd.cmd.user.id.to_string()).await?;
    let mut bounty = Bounty::new().await?;
    let uid = msg.mentions.iter().map(|x|x.id).collect::<Vec<_>>();
    let submit = BountySubmit::new(bnd, false, mem,
    uid, &mut pg, &bounty, &link, Methode::new(methode.unwrap()), BBQ::new(bbq.unwrap())?, Category::new(category.unwrap())?).await?;
    if !submit.cooldown(&mut bounty){
        return Err(MyErr::Custom("The Bounty You selected is on cooldown or disabled".to_owned()));
    }
    ChannelId::new(bnd.init.bounty.judge_ch).send_message(&bnd.ctx.http, CreateMessage::new().embed(submit.embed()).components(submit.button())).await?;
    msg.reply(&bnd.ctx.http, "Your bounty is already submitted to Judge").await?;
    submit.save(&bnd.user().id.to_string()).await;
    bounty.cooldown(bnd).await?;
    bounty.save().await?;
    pg.close().await;
    Ok(())
}
pub(super) async fn submit(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    let url = &bnd.cmd.data.resolved.attachments.values()
        .next().ok_or(MyErr::Custom("Cant get attachment".to_owned()))?.url;
    let mut mention = "";
    let mut methode ="";
    let mut category = "";
    let mut bbq = "";
    for i in Components::sub_options(bnd)?{
        if let CommandDataOptionValue::String(x)=&i.value{
            match i.name.as_str(){
                "category"=>{category=x.as_str()},
                "methode"=>{methode=x.as_str()},
                "mentions"=>{mention=x.as_str()},
                "bbq"=>{bbq=x.as_str()},
                _=>{continue;}
            }
        }
    }
    let cat = Category::new(category.parse::<u8>().unwrap())?;
    let bb = BBQ::new(bbq.parse::<u8>().unwrap())?;
    let met = Methode::new(methode.parse::<u8>().unwrap());
    let member = *bnd.cmd.member.clone().unwrap();
    let mut pg = PgConn::create(bnd.init, member.user.id.to_string()).await?;
    let mut bounty = Bounty::new().await?;
    let submit = BountySubmit::new(bnd, false, member, mentions(mention)?,&mut pg, &bounty, &url, met, bb, cat).await?;
    if !submit.cooldown(&mut bounty){
        return Err(MyErr::Custom("The Bounty You selected is on cooldown or disabled".to_owned()));
    }
    Components::response(bnd, "Your bounty is already submitted to Judge", false).await?;
    ChannelId::new(bnd.init.bounty.judge_ch).send_message(&bnd.ctx.http, CreateMessage::new().embed(submit.embed()).components(submit.button())).await?;
    submit.save(&bnd.user().id.to_string()).await;
    bounty.cooldown(bnd).await?;
    bounty.save().await?;
    pg.close().await;
    Ok(())
}



#[hertz::hertz_button_normal(0,false)]
async fn button(bnd:&ComponentBundle<'_>)->Result<(),MyErr>{
    let mut code = bnd.cmd.data.custom_id.split("_");
    let user = code.nth(1).ok_or(MyErr::Custom("cant get user id in custom id".to_owned()))?;
    let state = code.nth(2).ok_or(MyErr::Custom("cant get the button state in custom id".to_owned()))?;
    let mut submit = BountySubmit::open(user).await
        .ok_or(MyErr::Custom("submit data doesnt exist on cache".to_owned()))?;
    if state == "r"{
        //rejected
        let ch = ChannelId::new(bnd.init.bounty.receptionist_ch);
        submit.delete().await;
        ch.send_message(&bnd.ctx.http, CreateMessage::new().content(format!("<@{}> bounty is rejected by {}"
            ,user,&bnd.cmd.user.name))).await?;
        return Ok(());
    }
    //accepted
    let title = BountyTitle::new().await?;
    submit.title(bnd, &title).await?;
    let mut pg = PgConn::create(bnd.init, user.to_owned()).await?;
    submit.reward(false, bnd, &mut pg).await?;
    ChannelId::new(bnd.init.bounty.conquered_ch)
        .send_message(&bnd.ctx.http, CreateMessage::new().embed(submit.embed())).await?;
    submit.delete().await;
    pg.close().await;
    Ok(())
}
