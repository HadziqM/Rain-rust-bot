use crate::all::*;

pub struct ChangePassword;
pub struct AddPsn;

#[async_trait]
impl CommandInteractionTrait for ChangePassword {
    fn name(&self) -> String {
        "change_password".to_string()
    }
    fn command(&self) -> serenity::all::CreateCommand {
        AppReg::normal_slash(self.name(), "change your in game account password").add_option(
            CreateCommandOption::new(CommandOptionType::String, "password", "your new password")
                .required(true),
        )
    }
    async fn handle_int(
        &self,
        app: Arc<App>,
        cmd: CommandInteraction,
        ctx: Context,
    ) -> MyResult<()> {
        let data = app.only_register_user(&cmd.user).await?;
        if let CommandDataOptionValue::String(password) = &cmd.data.options.first().unwrap().value {
            app.db.change_password(password, data.uid).await?;
            Components::response(&cmd, &ctx, "Password changed", true).await?;
        }
        Ok(())
    }
}

#[async_trait]
impl CommandInteractionTrait for AddPsn {
    fn name(&self) -> String {
        "add_psn".to_string()
    }

    fn command(&self) -> serenity::all::CreateCommand {
        AppReg::normal_slash(self.name(), "add your psn account")
    }

    async fn handle_int(
        &self,
        _app: Arc<App>,
        cmd: CommandInteraction,
        _ctx: Context,
    ) -> MyResult<()> {
        let res = CreateInteractionResponse::Modal(
            CreateModal::new(self.name(), "Register/Override PSN ID").components(vec![
                CreateActionRow::InputText(
                    CreateInputText::new(InputTextStyle::Short, "PSN_ID", "psn_id")
                        .required(true)
                        .placeholder("For console player (You can Leave it Empty)"),
                ),
            ]),
        );
        cmd.create_response(&_ctx.http, res).await?;
        Ok(())
    }
}

#[async_trait]
impl ButtonInteractionTrait for AddPsn {
    fn name_btn(&self) -> String {
        self.name()
    }
    async fn handle_button(
        &self,
        _app: Arc<App>,
        cmd: ComponentInteraction,
        _ctx: Context,
    ) -> MyResult<()> {
        let res = CreateInteractionResponse::Modal(
            CreateModal::new(self.name(), "Register/Override PSN ID").components(vec![
                CreateActionRow::InputText(
                    CreateInputText::new(InputTextStyle::Short, "PSN_ID", "psn_id")
                        .required(true)
                        .placeholder("For console player (You can Leave it Empty)"),
                ),
            ]),
        );
        cmd.create_response(&_ctx.http, res).await?;
        Ok(())
    }
}

#[async_trait]
impl ModalInteractionTrait for AddPsn {
    fn name_mdl(&self) -> String {
        self.name()
    }
    async fn handle_modal(
        &self,
        _app: Arc<App>,
        cmd: ModalInteraction,
        _ctx: Context,
    ) -> MyResult<()> {
        let data = _app.only_register_user(&cmd.user).await?;

        if let Some(psn) = cmd.data.components.first() {
            if let Some(ActionRowComponent::InputText(psn_x)) = psn.components.first() {
                if let Some(psn_id) = &psn_x.value {
                    _app.db.change_psn(psn_id, data.uid).await?;
                    Components::response(&cmd, &_ctx, "PSN successfully added/changed", true)
                        .await?
                }
            }
        }

        Ok(())
    }
}
