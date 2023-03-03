use crate::{AppReg, Init};
use serenity::all::CommandOptionType;
use serenity::builder::{CreateCommand, CreateCommandOption};

pub mod add;
pub mod config;
pub mod interface;
pub mod market;
pub mod monitor;
pub mod password;
pub mod purge;
pub mod query;
pub mod test;


pub fn reg(init: &Init) -> Vec<CreateCommand> {
    let tests = AppReg::admin_slash("test", "test bounty title given trigger").add_option(
        CreateCommandOption::new(
            CommandOptionType::String,
            "trigger",
            "the trigger listed on bounty_title.json",
        )
        .required(true)
        .set_autocomplete(true),
    );
    let save = AppReg::admin_slash("mod_pass", "reset someone password given username")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "username",
                "username you want to reset",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "password",
                "reset password given input",
            )
            .required(true),
        );
    let user_op = CreateCommandOption::new(CommandOptionType::User, "user", "mention the customer")
        .required(true);
    let att_op = CreateCommandOption::new(
        CommandOptionType::Attachment,
        "json",
        "send your matching .json file",
    )
    .required(true);
    let price_op = CreateCommandOption::new(
        CommandOptionType::Integer,
        "price",
        "item price (all) not single",
    );
    let count_op = CreateCommandOption::new(
        CommandOptionType::Integer,
        "unit",
        "quantity of the item send max 65025",
    )
    .required(true);
    let item_op = CreateCommandOption::new(
        CommandOptionType::String,
        "item ",
        "search one of ~17770 item on the list",
    )
    .required(true)
    .set_autocomplete(true);
    let item = AppReg::admin_slash(
        "send",
        "send item trough distribution then deduct their bounty coin",
    )
    .add_option(
        CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "item",
            "send with item category",
        )
        .add_sub_option(user_op.to_owned())
        .add_sub_option(item_op.to_owned())
        .add_sub_option(count_op.to_owned())
        .add_sub_option(price_op.to_owned()),
    )
    .add_option(
        CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "melee",
            "send with melee weapon category",
        )
        .add_sub_option(user_op.to_owned())
        .add_sub_option(item_op.to_owned())
        .add_sub_option(price_op.to_owned()),
    )
    .add_option(
        CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "ranged",
            "send with ranged weapon category",
        )
        .add_sub_option(user_op.to_owned())
        .add_sub_option(item_op.to_owned())
        .add_sub_option(price_op.to_owned()),
    )
    .add_option(
        CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "head",
            "send with head armor category",
        )
        .add_sub_option(user_op.to_owned())
        .add_sub_option(item_op.to_owned())
        .add_sub_option(price_op.to_owned()),
    )
    .add_option(
        CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "arms",
            "send with arms armor category",
        )
        .add_sub_option(user_op.to_owned())
        .add_sub_option(item_op.to_owned())
        .add_sub_option(price_op.to_owned()),
    )
    .add_option(
        CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "chest",
            "send with chest armor category",
        )
        .add_sub_option(user_op.to_owned())
        .add_sub_option(item_op.to_owned())
        .add_sub_option(price_op.to_owned()),
    )
    .add_option(
        CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "waist",
            "send with waist armor category",
        )
        .add_sub_option(user_op.to_owned())
        .add_sub_option(item_op.to_owned())
        .add_sub_option(price_op.to_owned()),
    )
    .add_option(
        CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "leg",
            "send with leg armor category",
        )
        .add_sub_option(user_op.to_owned())
        .add_sub_option(item_op.to_owned())
        .add_sub_option(price_op.to_owned()),
    );
    let monitor = AppReg::admin_slash("monitor", "toggle monitor server in info channel");
    let add = AppReg::admin_slash("add", "add bounty coint or gacha")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommandGroup,
                "bounty",
                "add bounty point to player",
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "mentions",
                    "add bounty coin to mentioned player",
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::User,
                        "mention",
                        "mention the user",
                    )
                    .required(true),
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::Integer,
                        "amount",
                        "the amount gifted, can be negative",
                    )
                    .required(true),
                ),
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "all",
                    "send bounty to all player",
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::Integer,
                        "amount",
                        "the amount gifted, can be negative",
                    )
                    .required(true),
                ),
            ),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommandGroup,
                "gacha",
                "add gacha point to player",
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "mentions",
                    "add gacha ticket to mentioned player",
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::User,
                        "mention",
                        "mention the user",
                    )
                    .required(true),
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::Integer,
                        "amount",
                        "the amount gifted, can be negative",
                    )
                    .required(true),
                ),
            )
            .add_sub_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "all",
                    "send gacha ticket to all player",
                )
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::Integer,
                        "amount",
                        "the amount gifted, can be negative",
                    )
                    .required(true),
                ),
            ),
        );
    let mut config = AppReg::admin_slash("config", "change configuration of some bot feature")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "tag",
                "config for tag commands",
            )
            .add_sub_option(att_op.to_owned()),
        );
    if init.bot_config.gacha {
        config = config.add_option(
            CreateCommandOption::new(
                CommandOptionType::SubCommand,
                "gacha",
                "send your gacha.json file",
            )
            .add_sub_option(att_op.to_owned()),
        );
    }
    if init.bot_config.server_market {
        config = config
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "market",
                    "send your market.json file for trading market",
                )
                .add_sub_option(att_op.to_owned()),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "meal",
                    "send your meal.json file for trading market",
                )
                .add_sub_option(att_op.to_owned()),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "trading",
                    "send your trading.json file for trading market",
                )
                .add_sub_option(att_op.to_owned()),
            );
    }
    if init.bot_config.bounty {
        config = config
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "bounty",
                    "send your gacha.json file",
                )
                .add_sub_option(att_op.to_owned()),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "bounty_refresh",
                    "send your gacha_refresh.json file",
                )
                .add_sub_option(att_op.to_owned()),
            )
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::SubCommand,
                    "bounty_title",
                    "bounty_title.json config to set title",
                )
                .add_sub_option(att_op.to_owned()),
            );
    }
    vec![
        AppReg::admin_slash("interface", "mhfz user interface"),
        AppReg::admin_slash("purge", "purge user binded and register data on database").add_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "mention user you want to purge",
            )
            .required(true),
        ),
        save,
        item,
        config,
        monitor,
        add,
        tests
    ]
}
