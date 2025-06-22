use crate::all::*;
use std::str::FromStr;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

pub mod credential;
pub mod registration;

command_reg![Account];
button_reg![registration::BindAccount, registration::RegisterAccount];
modal_reg![registration::BindAccount, registration::RegisterAccount];

#[derive(Clone, Debug, EnumIter, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum CommandAccount {
    Create,
    Bind,
    ChangePassword,
    LinkPsn,
    UnlinkPsn,
}

impl CommandAccount {
    fn description(&self) -> &'static str {
        match self {
            Self::ChangePassword => "Change your account password",
            Self::Create => "Create Game account here",
            Self::Bind => "Bind Existing game account",
            Self::LinkPsn => "Link or Change Psn ID here",
            Self::UnlinkPsn => "Unlink Psn ID",
        }
    }

    fn subcommand(&self) -> CreateCommandOption {
        match self {
            Self::ChangePassword => AppReg::subcommand(self.to_string(), self.description())
                .add_string_choice("password", "your new password"),
            Self::LinkPsn => AppReg::subcommand(self.to_string(), self.description())
                .add_string_choice("psn", "your new PSN ID"),
            _ => AppReg::subcommand(self.to_string(), self.description()),
        }
    }

    async fn handle(&self, app: Arc<App>, cmd: CommandInteraction, ctx: Context) -> MyResult<()> {
        match self {
            Self::Bind => registration::registration(cmd, ctx, false).await,
            Self::Create => registration::registration(cmd, ctx, false).await,
            Self::ChangePassword => credential::change_password(app, cmd, ctx).await,
            Self::LinkPsn => credential::add_psn(app, cmd, ctx).await,
            Self::UnlinkPsn => credential::unlink_psn(app, cmd, ctx).await,
        }
    }
}

pub struct Account;

#[async_trait]
impl CommandInteractionTrait for Account {
    fn name(&self) -> &'static str {
        "account"
    }
    fn command(&self) -> CreateCommand {
        let mut x = AppReg::normal_slash(self.name(), "account configuration");

        for i in CommandAccount::iter() {
            x = x.add_option(i.subcommand());
        }
        x
    }

    async fn handle_int(
        &self,
        app: Arc<App>,
        cmd: CommandInteraction,
        ctx: Context,
    ) -> MyResult<()> {
        for data in &cmd.data.options {
            if let CommandDataOptionValue::SubCommand(_) = &data.value {
                let name = &data.name;
                let command = CommandAccount::from_str(name).unwrap();
                return command.handle(app, cmd, ctx).await;
            }
        }
        Ok(())
    }
}
