use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
    CommandDataOptionValue, ApplicationCommandInteraction,
};
use serenity::prelude::Context;

pub async fn run(options: &[CommandDataOption],ctx:&Context, command:&ApplicationCommandInteraction) {
    let option = options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    let out;
    if let CommandDataOptionValue::User(user, _member) = option {
        out = format!("{}'s id is {}", user.tag(), user.id);
    } else {
        out = "Please provide a valid user".to_string();
    };
    if let Err(why) = command.create_interaction_response(&ctx.http, |resp| {
        resp.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|msg|msg.content(out.as_str()))
    }).await{
        println!("cannot respond to slash command: {}",why)
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("id").description("Get a user id").create_option(|option| {
        option
            .name("id")
            .description("The user to lookup")
            .kind(CommandOptionType::User)
            .required(true)
    })
}
