use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use super::config::Init;

pub mod card;
pub mod account;

#[derive(Debug)]
pub struct PgConnection<'a> {
    pub init: &'a Init,
    pub did: &'a str,
    pub pool:Pool<Postgres>
}

impl<'a> PgConnection<'a> {
    pub async fn create(init:&'a Init, did:&'a str)->Result<PgConnection<'a>,sqlx::Error>{
        let pg = PgConnection{
            init,did,
            pool:connection(init).await?
        };
        Ok(pg)
    }
    pub async fn close(&mut self){
        self.pool.close().await
    }
}


pub async fn connection(init:&Init) -> Result<Pool<Postgres>, sqlx::Error> {
    let url = format!("postgres://postgres:{}@{}:{}/{}",
        init.postgress.password,
        init.postgress.host,
        init.postgress.port,
        init.postgress.database);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url.as_str()).await?;
    Ok(pool)
}

#[cfg(test)]
mod postgres_test{
    use super::connection;
    use crate::get_config;

    #[tokio::test]
    async fn test_connection() {
        let mut res = "success".to_string();
        if let Err(why) = connection(&get_config().unwrap()).await{
            res = "fail".to_string();
            println!("{why}")
        };
        assert_eq!(&res,"success")
    }
}
