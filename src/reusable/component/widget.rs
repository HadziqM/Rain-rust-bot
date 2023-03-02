use serenity::all::{ButtonStyle, Message, CommandDataOptionValue, CommandDataOption};
use serenity::builder::{CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse, CreateMessage};
use serenity::model::prelude::ReactionType;
use super::{Components,Mytrait,MyErr};
use crate::{Mybundle,MsgBundle,SlashBundle};

impl Components{
    pub fn normal_button(name:&str,custom_id:&str,style:ButtonStyle,emoji:&str)->CreateButton{
        let mut b = CreateButton::new(custom_id).label(name).style(style);
        if let Ok(emj)=emoji.parse::<ReactionType>(){
            b = b.emoji(emj);
        }
        b
    }
    pub fn interaction_response(content:&str,ephemeral:bool)->CreateInteractionResponse{
        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(content).ephemeral(ephemeral)
            )
    }
    pub async fn response<T:Mybundle>(bnd:&T,content:&str,ephemeral:bool)->Result<(),MyErr>{
        let cmd = bnd.cmd();
        Ok(cmd.response(bnd.ctx(), Components::interaction_response(content, ephemeral)).await?)
    }
    pub async fn response_adv<T:Mybundle>(bnd:&T,content:CreateInteractionResponse)->Result<(),MyErr>{
        let cmd = bnd.cmd();
        Ok(cmd.response(bnd.ctx(), content).await?)
    }
    pub async fn edit<T:Mybundle>(bnd:&T,content:&str)->Result<(),MyErr>{
        let cmd = bnd.cmd();
        let rply = EditInteractionResponse::new().content(content);
        Ok(cmd.edit(bnd.ctx(), rply).await?)
    }
    pub async fn edit_adv<T:Mybundle>(bnd:&T,content:EditInteractionResponse)->Result<(),MyErr>{
        let cmd = bnd.cmd();
        Ok(cmd.edit(bnd.ctx(), content).await?)
    }
    pub async fn msg(bnd:&MsgBundle<'_>,content:&str)->Result<Message,MyErr>{
        if content.len() >= 2000{
            return Err(MyErr::Custom("the result is higher than 2000 char,a nd discord doesnt allow it".to_string()));
        }
        Ok(bnd.msg.channel_id.send_message(&bnd.ctx.http, CreateMessage::new().content(content)).await?)
    }
    pub fn msg_reply_raw(bnd:&MsgBundle<'_>)->CreateMessage{
        let mut msg = CreateMessage::new();
        match &bnd.msg.referenced_message{
            Some(m)=>{
                msg = msg.reference_message(&**m);
            }
            None=>{
                msg = msg.reference_message(bnd.msg);
            }
        };
        msg
    }
    pub async fn msg_adv(bnd:&MsgBundle<'_>,content:CreateMessage)->Result<Message,MyErr>{
        Ok(bnd.msg.channel_id.send_message(&bnd.ctx.http, content).await?)
    }
    pub fn sub_options<'a>(bnd:&'a SlashBundle<'_>)->Result<&'a Vec<CommandDataOption>,MyErr>{
        for data in &bnd.cmd.data.options{
            if let CommandDataOptionValue::SubCommand(x) = &data.value{
                return Ok(x);
            }
        }
        Err(MyErr::Custom("cant find subcommand".to_string()))
    }
}
