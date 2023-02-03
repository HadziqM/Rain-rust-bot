use serenity::all::ButtonStyle;
use serenity::builder::{CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::prelude::ReactionType;

use super::Components;

impl Components{
    pub fn normal_button(name:&str,custom_id:&str,style:ButtonStyle,emoji:&str)->CreateButton{
        let mut b = CreateButton::new(custom_id);
        b.label(name);
        b.style(style);
        if let Ok(emj)=emoji.parse::<ReactionType>(){
            b.emoji(emj);
        }
        b
    }
    pub fn interaction_response(content:&str,ephemeral:bool)->CreateInteractionResponse{
        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(content).ephemeral(ephemeral)
            )
    }
}

