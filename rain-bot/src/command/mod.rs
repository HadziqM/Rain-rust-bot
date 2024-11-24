pub mod register;

use crate::all::*;

use crate::reg;

reg![
    reg_command,
    CommandInteractionTrait,
    register::reg_command()
];
reg![reg_button, ButtonInteractionTrait, register::reg_button()];
reg![reg_modal, ModalInteractionTrait, register::reg_modal()];
