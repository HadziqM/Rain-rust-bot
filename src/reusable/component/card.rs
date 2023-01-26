use serenity::{builder::CreateInteractionResponse, model::{prelude::interaction::InteractionResponseType, user::User}};

use crate::reusable::{postgress::card::Card, utils::color};

impl Card{
    pub fn get_path(&self)->(String,String){
        let iconlist = vec!["./icon/GS.png", "./icon/HS.png", "./icon/H.png", "./icon/L.png", "./icon/SS.png", "./icon/LB.png", "./icon/DS.png","./icon/LS.png", "./icon/HH.png", "./icon/GL.png", "./icon/B.png", "./icon/T.png", "./icon/SAF.png", "./icon/MS.png"];
        let pt = iconlist[self.weapon_type as usize].to_string();
        let gh = pt.to_owned().split("/").last().unwrap().to_string();
        (pt,gh)
    }
    fn g_name(&self)->String{
        match &self.guild_name {
            Some(x) => x.to_owned(),
            None => "No guild".to_string(),
        }
    }
    fn g_id(&self)->String{
        match self.guild_id {
            Some(x) => x.to_string(),
            None => "No id".to_string(),
        }
    }
    fn hrp(&self)->u8{
    if self.hrp==999{
            return 7;
        }else if self.hrp>299{
            return 6;
        }else if self.hrp>99{
            return 5;
        }else if self.hrp>50{
            return 4;
        }else if self.hrp>30{
            return 3;
        }else if self.hrp>1{
            return 2;
        }
        1
    }
    fn last_login(&self)->String{
        format!("<t:{}:R>",self.login)
    }
    pub fn card<'a,'b>(&self,m:&'a mut CreateInteractionResponse<'b>,user:&User,path:&'b str)->&'a mut CreateInteractionResponse<'b>{
        m.kind(InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|d|{
            d.embed(|emb|{
                emb.title(self.name.as_str()).fields(vec![
                    ("User",&format!("username: {}\nuser_id: {}\nchar_id: {}\nlast_login: {}",&self.username,self.user_id,self.char_id,self.last_login()),false),
                    ("Character",&format!("HR: {}\nGR: {}",self.hrp(),self.gr),false),
                    ("Guild",&format!("name: {}\nguild_id: {}",&self.g_name(),&self.g_id()),false)
                ]).footer(|f|f.text(&format!("character owned by {}",user.name)).icon_url(user.face()))
                    .colour(color("ff", "55", "00")).thumbnail(&format!("attachment://{}",self.get_path().1))
            }).add_file(path)

        })
    }
}
