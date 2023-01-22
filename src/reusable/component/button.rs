use serenity::model::application::component::ButtonStyle;
use serenity::builder::CreateButton;

pub fn normal_button(name:&str,custom_id:&str,style:ButtonStyle)->CreateButton{
    let mut b = CreateButton::default();
    b.custom_id(custom_id);
    b.label(name);
    b.style(style);
    b
}
