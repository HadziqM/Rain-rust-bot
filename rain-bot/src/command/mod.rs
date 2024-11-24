use crate::all::*;
use crate::reg;

mod register;
mod test;

reg![
    reg_command,
    CommandInteractionTrait,
    register::reg_command(),
    test::reg_command()
];
reg![reg_button, ButtonInteractionTrait, register::reg_button()];
reg![reg_modal, ModalInteractionTrait, register::reg_modal()];
