use serenity::{builder::CreateCommand, all::CommandType, model::Permissions};



pub struct AppReg;

impl AppReg{
    pub fn user_context(name:&str)->CreateCommand{
        CreateCommand::new(name).kind(CommandType::User)
    }
    pub fn message_context(name:&str)->CreateCommand{
        CreateCommand::new(name).kind(CommandType::Message)
    }
    pub fn normal_slash(name:&str,desc:&str)->CreateCommand{
        CreateCommand::new(name).description(desc)
    }
    pub fn admin_slash(name:&str,desc:&str)->CreateCommand{
        CreateCommand::new(name).description(desc).default_member_permissions(Permissions::ADMINISTRATOR)
    }
}
