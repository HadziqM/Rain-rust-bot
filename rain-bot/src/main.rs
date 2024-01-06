pub mod config;
pub mod error;
pub mod utils;
pub mod setup;
pub mod command;

use error::MyErr;
use config::Init;
use setup::{Setup,Context,AppData};



#[tokio::main]
async fn main() -> Result<(),MyErr> {
    let framework = Setup::new(vec![]).await?;
    framework.run().await;
    Ok(())
}
