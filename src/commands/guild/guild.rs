use serenity::all::*;
use crate::{MyErr,Mytrait,Mybundle,SlashBundle,Components,Reg,PgConn};

#[hertz::hertz_slash_reg(20,false)]
async fn slash(bnd:&SlashBundle<'_>,reg:&Reg<'_>)->Result<(),MyErr>{
    let mut name = "";
    let mut gid = 0;
    for i in &bnd.cmd.data.options{
        if let CommandDataOptionValue::SubCommand(x) = &i.value{
            name = i.name.as_str();
            for j in x{
                if let CommandDataOptionValue::Integer(y) = &j.value{
                    gid = *y;
                }
            }
        }
    }
    match name{
        "join"=>join(bnd, reg, gid).await?,
        _ =>see(bnd, reg, gid).await?
    }
    Ok(())
}

#[hertz::hertz_auto]
async fn auto(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    for i in Components::sub_options(bnd)?{
        if let CommandDataOptionValue::Autocomplete { kind:_, value } = &i.value{
            let mut pg = PgConn::create(bnd.init,bnd.cmd.user.id.to_string()).await?;
            let list = pg.guild_list().await?;
            let auto = list.iter().filter_map(|x|{
                let name = x.name.to_lowercase();
                let foc = value.to_lowercase();
                if name.contains(&foc) || name.starts_with(&foc){
                    return Some(AutocompleteChoice{name:x.name.to_owned(),value:serde_json::Value::Number(x.id.into())})
                }
                None
            }).collect::<Vec<_>>();
            bnd.cmd.create_response(&bnd.ctx.http, CreateInteractionResponse::Autocomplete(CreateAutocompleteResponse::new().set_choices(auto))).await?;
            pg.close().await;
        }
    }
    Ok(())
}

async fn join(bnd:&SlashBundle<'_>,reg:&Reg<'_>,gid:i64)->Result<(),MyErr>{
    reg.pg.guild_join(gid, reg.cid).await?;
    Components::response(bnd, "succesfully joined", true).await?;
    Ok(())
}
async fn see(bnd:&SlashBundle<'_>,reg:&Reg<'_>,gid:i64)->Result<(),MyErr>{
    let data = reg.pg.guild_search(gid).await?;
    let embed = data.0.embed(data.1, bnd).await?;
    Components::response_adv(bnd, CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().embed(embed))).await?;
    Ok(())
}
use crate::reusable::postgress::guild::Guild;
impl Guild{
    async fn embed(&self,count:i64,bnd:&SlashBundle<'_>)->Result<CreateEmbed,MyErr>{
        let time = &self.created.timestamp();
        let leader;
        match &self.discord_id{
            Some(x) =>{
                leader = format!("<@{}>",x);
            }
            None=>{
                leader = "None".to_owned();
            }
        }
        let mut emb = CreateEmbed::new().title(&self.name).fields(vec![
            ("General Details",format!(" 🆔 Guild_id: {}\n 🏛️ Created: <t:{time}:R> \n 🧑 Member Count: {count}/60 \n 🎖️ Rank Point : {}"
                ,self.id,self.rank_rp),false),
            ("Leader Details",format!(" 🆔 Leader_id: {}\n 🏷️ Leader Name: {} \n 🎮 Leader Discord: {leader}"
                ,self.leader_id,self.lead_name),false),
        ]);
        if leader.as_str() != "None"{
            let user = UserId::new(self.discord_id.clone().unwrap().parse::<u64>().unwrap()).to_user(&bnd.ctx.http).await?;
            emb = emb.footer(CreateEmbedFooter::new(format!("Owned by {}",&user.name)).icon_url(user.face()));
        }
        Ok(emb)
    }
}
