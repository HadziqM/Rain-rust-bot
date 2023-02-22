use serenity::builder::CreateEmbed;
use serenity::all::*;
use crate::reusable::image_edit::gacha::GachaImage;
use crate::reusable::postgress::gacha::GachaPg;
use crate::{Reg,SlashBundle,MyErr,Components};
use crate::reusable::utils::Color;
use crate::reusable::component::gacha::Gacha;
use crate::{Mybundle,Mytrait};


fn create_embed(user:&User,pg:&GachaPg)->CreateEmbed{
    CreateEmbed::new().title("Mhfz Gacha Result").description(format!("Pity Count: {}\nTicket Remaining: {}",pg.pity,pg.ticket))
        .author(CreateEmbedAuthor::new(&user.name).icon_url(user.face())).image("attachment://gacha.jpg").color(Color::Random.throw())
}

#[hertz::hertz_slash_reg(60,true)]
async fn slash(bnd:&SlashBundle<'_>,reg:&Reg<'_>)->Result<(),MyErr>{
    let mut multi = false;
    for i in &bnd.cmd.data.options{
        if let CommandDataOptionValue::SubCommand(_) = &i.value{
            if i.name == "multi"{
                multi = true;
            }
        }
    }
    let data = reg.pg.get_pity().await?;
    let gacha = Gacha::new().await?;
    let image = GachaImage::new(&bnd.cmd.user.static_avatar_url().unwrap_or(bnd.cmd.user.default_avatar_url())).await?;
    let raw;
    let g_pg;
    let g_data;
    let cost = ||{if multi{return 110;}10};
    if data.ticket<cost(){
            return Err(MyErr::Custom(format!("insufficient ticket, you only have {} ticket and its need {} ticket to pull, so you need to collect {} more",data.ticket,cost(),cost()-data.ticket)));
    }
    if !multi{
        let result = gacha.single_pull(&data);
        g_pg = result.0;
        raw =image.single_pull(&result.1,bnd.pedia,bnd.image).await?;
        g_data = vec![result.1];
    }else{
        let result = gacha.multi_pull(&data);
        g_pg=result.0;
        raw = image.multi_pull(result.1.clone(),bnd.pedia,bnd.image).await?;
        g_data = result.1;
    }
    let code:Vec<_> = g_data.iter().map(|e|e.code.to_owned()).collect();
    reg.pg.send_distrib(&g_pg,&code, reg.cid, bnd.pedia).await?;
    let att = CreateAttachment::bytes(raw, "gacha.jpg");
    let embed = create_embed(&bnd.cmd.user, &g_pg);
    let content = EditInteractionResponse::new()
        .new_attachment(att).embed(embed);
    Components::edit_adv(bnd, content).await?;
    Ok(())
}
