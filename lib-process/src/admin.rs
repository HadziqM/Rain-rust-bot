use super::*;
use common::setting::SettingList;
use std::str::FromStr;

impl App {
    pub async fn command_change_setting(&self, byte: Vec<u8>, name: &str) -> MyResult<()> {
        let setting = SettingList::from_str(name).unwrap();

        let mut all = self.setting.write().await;
        setting
            .validete_and_change(&byte, &mut all)
            .map_err(|x| MyError::from(x.as_str()))?;

        tokio::fs::write(setting.path(), byte).await?;
        Ok(())
    }
}
