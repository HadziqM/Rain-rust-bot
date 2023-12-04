use serenity::{builder::{CreateCommand, CreateCommandOption}, all::CommandType, model::Permissions};



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
    pub fn subcommand(name:&str,desc:&str)->CreateCommandOption{
        CreateCommandOption::new(serenity::all::CommandOptionType::SubCommand, name, desc)
    }
    pub fn user_option(name:&str,desc:&str)->CreateCommandOption{
        CreateCommandOption::new(serenity::all::CommandOptionType::User, name, desc)
    }
    pub fn int_option(name:&str,desc:&str)->CreateCommandOption{
        CreateCommandOption::new(serenity::all::CommandOptionType::Integer, name, desc)
    }
    pub fn att_option(name:&str,desc:&str)->CreateCommandOption{
        CreateCommandOption::new(serenity::all::CommandOptionType::Attachment, name, desc)
    }
    pub fn str_option(name:&str,desc:&str)->CreateCommandOption{
        CreateCommandOption::new(serenity::all::CommandOptionType::String, name, desc)
    }
}
