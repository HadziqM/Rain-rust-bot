use serenity::model::application::component::ButtonStyle;
use serenity::builder::CreateButton;
use serenity::model::prelude::ReactionType;

pub fn normal_button(name:&str,custom_id:&str,style:ButtonStyle,emoji:ReactionType)->CreateButton{
    let mut b = CreateButton::default();
    b.custom_id(custom_id);
    b.label(name);
    b.style(style);
    b.emoji(emoji);
    b
}
