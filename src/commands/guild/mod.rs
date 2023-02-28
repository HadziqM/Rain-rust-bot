use serenity::all::*;

pub mod guild;

pub fn reg() -> Vec<CreateCommand> {
    let gname = CreateCommandOption::new(
        CommandOptionType::Integer,
        "guild",
        "search the guild you want to search",
    )
    .set_autocomplete(true)
    .required(true);
    let gcommand = CreateCommand::new("guild")
        .description("guild desc")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "info",
                "see guild informations",
            )
            .add_sub_option(gname.to_owned()),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "join",
                "join selected guild if you dont have one",
            )
            .add_sub_option(gname.to_owned()),
        );
    vec![gcommand]
}
