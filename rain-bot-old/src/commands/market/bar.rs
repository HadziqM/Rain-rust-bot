use crate::{MyErr,Mytrait,Mybundle,SlashBundle,ModalBundle,Components};
use serenity::all::*;
use crate::reusable::utils::Color;

pub(super) async fn slash(bnd:&SlashBundle<'_>)-> Result<(),MyErr>{
    let modal = CreateInteractionResponse::Modal(CreateModal::new("bar", "Bar Request Board")
        .components(vec![CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Paragraph,"requested item list", "item")),
        CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Short,"range price", "price"))]));
    Components::response_adv(bnd, modal).await?;
    Ok(())
}

#[hertz::hertz_modal_normal(0,false)]
async fn modal(bnd:&ModalBundle<'_>)->Result<(),MyErr>{
    let mut item = "";
    let mut price = "";
    for data in &bnd.cmd.data.components{
        let arow = data.components.first().ok_or(MyErr::Custom("cant get first component".to_owned()))?;
        if let ActionRowComponent::InputText(x) = arow{
            match x.custom_id.as_str(){
                "item"=>{
                    item = x.value.as_str();
                }
                "price"=>{
                    price = x.value.as_str();
                }
                _ => {continue;}
            }
        }
    }
    let embed = CreateEmbed::new().title("Bar Request").color(Color::Blue.throw())
        .description(item).field("Requested price", price, false).author(
            CreateEmbedAuthor::new(&bnd.cmd.user.name).icon_url(bnd.cmd.user.face()));
    Components::response(bnd, "Requested", true).await?;
    let msg = ChannelId::new(bnd.init.log_channel.request_channel).send_message(&bnd.ctx.http, 
        CreateMessage::new().embed(embed).content(format!("<@&{}> {}",bnd.init.server_role.bartender_role
            ,bnd.cmd.user.to_string()))).await?;
    msg.channel_id.create_public_thread(&bnd.ctx.http,
        msg.id,CreateThread::new(format!("{} Request Thread",&bnd.cmd.user.name))).await?;
    Ok(())
}
