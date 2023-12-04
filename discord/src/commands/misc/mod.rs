pub mod ferias;
pub mod tag;
pub mod gpt;

use crate::AppReg;
use serenity::all::*;

pub fn reg() -> Vec<CreateCommand> {
    let ferias = AppReg::normal_slash("ferias", "link you a ferias item").add_option(
        CreateCommandOption::new(
            CommandOptionType::String,
            "item",
            "the item you wanna search",
        )
        .set_autocomplete(true)
        .required(true),
    );
    let gpt = AppReg::normal_slash("gpt", "chat-gpt command")
        .add_option(
            AppReg::subcommand(
                "chat",
                "chat with chat-gpt (using free token so expected rate limit)",
            )
            .add_sub_option(AppReg::str_option("ask", "what you ask to chat-gpt").required(true)),
        )
        .add_option(
            AppReg::subcommand("image", "generate image with bot")
                .add_sub_option(
                    AppReg::int_option("quantity", "the number of image generated 1-10")
                        .min_int_value(1)
                        .max_int_value(10)
                        .required(true),
                )
                .add_sub_option(
                    AppReg::str_option("size", "the image size generated")
                        .required(true)
                        .add_string_choice("256x256", "256x256")
                        .add_string_choice("512x512", "512x512")
                        .add_string_choice("1024x1024", "1024x1024"),
                )
                .add_sub_option(
                    AppReg::str_option("description", "description of image you want to generate")
                        .required(true),
                ),
        );
    vec![ferias, gpt]
}
