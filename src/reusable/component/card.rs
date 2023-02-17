use serenity::all::{ButtonStyle, User};
use serenity::builder::{CreateActionRow, CreateEmbed, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse};

use crate::reusable::{postgress::card::Card, utils::color};

use crate::Components;

fn make_button()->CreateActionRow{
    CreateActionRow::Buttons(vec![
        Components::normal_button("use", "use", ButtonStyle::Primary, "👍"),
        Components::normal_button("next", "next", ButtonStyle::Success, "➡️")
    ])
}
impl Card{
    pub fn get_path(&self)->String{
        let iconlist = vec![
        "https://media.discordapp.net/attachments/1068440173479739393/1068440322977312868/GS.png",
        "https://media.discordapp.net/attachments/1068440173479739393/1068440324617281626/HS.png",
        "https://media.discordapp.net/attachments/1068440173479739393/1068440323501596792/H.png",
        "https://media.discordapp.net/attachments/1068440173479739393/1068440324931862599/L.png",
        "https://media.discordapp.net/attachments/1068440173479739393/1068440373548044348/SS.png",
        "https://media.discordapp.net/attachments/1068440173479739393/1068440325309345822/LB.png",
        "https://media.discordapp.net/attachments/1068440173479739393/1068440322260086794/DS.png",
        "https://media.discordapp.net/attachments/1068440173479739393/1068440372474302464/LS.png",
        "https://media.discordapp.net/attachments/1068440173479739393/1068440324088807466/HH.png",
        "https://media.discordapp.net/attachments/1068440173479739393/1068440322633383946/GL.png",
        "https://media.discordapp.net/attachments/1068440173479739393/1068440321907761162/B.png",
        "https://media.discordapp.net/attachments/1068440173479739393/1068440373757743154/T.png",
        "https://media.discordapp.net/attachments/1068440173479739393/1068440373132800080/SAF.png",
        "https://media.discordapp.net/attachments/1068440173479739393/1068440372709167174/MS.png"
        ];
        iconlist[self.weapon_type as usize].to_string()
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

    fn crete_embed(&self,user:&User)->CreateEmbed{
        CreateEmbed::new()
        .title(self.name.as_str()).fields(vec![
        ("User",&format!("username: {}\nuser_id: {}\nchar_id: {}\nlast_login: {}",&self.username,self.user_id,self.char_id,self.last_login()),false),
        ("Character",&format!("HR: {}\nGR: {}",self.hrp(),self.gr),false),
        ("Guild",&format!("name: {}\nguild_id: {}",&self.g_name(),&self.g_id()),false)
    ]).footer(CreateEmbedFooter::new(format!("owned by {}",user.name)).icon_url(user.face()))
        .colour(color("ff", "55", "00")).thumbnail(&self.get_path())
    }

    pub fn card(&self,user:&User,ephemeral:bool)->CreateInteractionResponse{
        CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
            .embed(self.crete_embed(user)).ephemeral(ephemeral))
    }
    pub fn bind(&self,user:&User)->CreateInteractionResponse{
        CreateInteractionResponse::Message(CreateInteractionResponseMessage::new()
            .embed(self.crete_embed(user)).components(vec![make_button()]))
    }
    pub fn edit_bind(&self,user:&User)->EditInteractionResponse{
        EditInteractionResponse::new().embed(self.crete_embed(user)).components(vec![make_button()])
    }
}
