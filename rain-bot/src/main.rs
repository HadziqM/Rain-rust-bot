pub mod config;
pub mod error;
pub mod utils;
pub mod setup;
pub mod command;
pub mod model;

use error::MyErr;
use config::Init;
use setup::{Setup,Context,ApplicationContext,AppData};



#[tokio::main]
async fn main() -> Result<(),MyErr> {
    let framework = Setup::new(command::reg()).await?;
    framework.run().await;
    Ok(())
}
