pub mod command;
pub mod config;
pub mod error;
pub mod model;
pub mod setup;
pub mod utils;

use config::Init;
use error::MyErr;
use setup::{AppData, ApplicationContext, Context, Setup};

#[tokio::main]
async fn main() -> Result<(), MyErr> {
    let command_list = command::reg();
    let framework = Setup::new(command_list).await?;
    framework.run().await;
    Ok(())
}
