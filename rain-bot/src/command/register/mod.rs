use crate::all::*;

mod credential;

command_reg![credential::ChangePassword, credential::AddPsn];
button_reg![credential::AddPsn];
modal_reg![credential::AddPsn];
