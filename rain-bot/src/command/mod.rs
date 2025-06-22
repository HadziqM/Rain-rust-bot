use crate::all::*;
use crate::reg;

mod account;
mod admin;
mod gacha;
mod test;

reg![
    reg_command,
    CommandInteractionTrait,
    account::reg_command(),
    test::reg_command(),
    admin::reg_command(),
    gacha::reg_command()
];
reg![reg_button, ButtonInteractionTrait, account::reg_button()];
reg![reg_modal, ModalInteractionTrait, account::reg_modal()];
reg![reg_message, MessageCommandTrait, admin::reg_message()];
