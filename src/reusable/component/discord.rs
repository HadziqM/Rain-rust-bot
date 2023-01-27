use serenity::{builder::CreateApplicationCommand, model::prelude::command::CommandType};


pub struct AppReg;

impl AppReg{
    pub fn user_context(name:&str)->CreateApplicationCommand{
        let mut app = CreateApplicationCommand::default();
        app.kind(CommandType::User).name(name);
        app
    }
    pub fn message_context(name:&str)->CreateApplicationCommand{
        let mut app = CreateApplicationCommand::default();
        app.kind(CommandType::Message).name(name);
        app
    }
    pub fn normal_slash(name:&str,desc:&str)->CreateApplicationCommand{
        let mut app = CreateApplicationCommand::default();
        app.name(name).description(desc);
        app
    }
}