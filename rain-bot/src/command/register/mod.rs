use crate::all::*;

mod create;
mod credential;
mod gacha;

command_reg![
    credential::ChangePassword,
    credential::AddPsn,
    create::RegisterAccount,
    create::BindAccount
];
button_reg![
    credential::AddPsn,
    create::BindAccount,
    create::RegisterAccount
];
modal_reg![
    credential::AddPsn,
    create::BindAccount,
    create::RegisterAccount
];
