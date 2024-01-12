use super::bitwise::BitwiseError;
use super::config::Init;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub mod account;
pub mod card;
pub mod custom;
pub mod gacha;
pub mod guild;
pub mod server;

#[derive(Debug)]
pub struct PgConn<'a> {
    pub(crate) init: &'a Init,
    pub(crate) did: String,
    pub(crate) pool: Pool<Postgres>,
}

impl<'a> PgConn<'a> {
    pub async fn create(init: &'a Init, did: String) -> Result<PgConn<'a>, BitwiseError> {
        let pg = PgConn {
            init,
            did,
            pool: connection(init).await?,
        };
        Ok(pg)
    }
    pub async fn close(&mut self) {
        self.pool.close().await
    }
    pub async fn re_connect(&mut self) -> Result<(), BitwiseError> {
        self.pool = connection(self.init).await?;
        Ok(())
    }
}

pub async fn connection(init: &Init) -> Result<Pool<Postgres>, sqlx::Error> {
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        init.postgress.user,
        init.postgress.password,
        init.postgress.host,
        init.postgress.port,
        init.postgress.database
    );
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url.as_str())
        .await?;
    Ok(pool)
}

// #[cfg(test)]
// mod postgres_test{
//     use super::connection;
//     use crate::get_config;
//
//     #[tokio::test]
//     async fn test_connection() {
//         let mut res = "success".to_string();
//         if let Err(why) = connection(&get_config().unwrap()).await{
//             res = "fail".to_string();
//             println!("{why}")
//         };
//         assert_eq!(&res,"success")
//     }
// }
