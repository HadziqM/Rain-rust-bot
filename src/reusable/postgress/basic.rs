use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use crate::CONFIG;
// use sqlx::mysql::MySqlPoolOptions;
// etc.


async fn connection() -> Result<Pool<Postgres>, sqlx::Error> {
    let url;
    unsafe{
        url = format!("postgres://postgres:{}@{}:{}/{}",
            &CONFIG.postgress.password,
            &CONFIG.postgress.host,
            &CONFIG.postgress.port,
            &CONFIG.postgress.database);
    }
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url.as_str()).await?;
    Ok(pool)
}

#[cfg(test)]
mod postgres_test{
    use crate::reusable::config::get_config;

    use super::*;

    #[tokio::test]
    async fn test_connection() {
        unsafe{
            CONFIG = get_config().unwrap()
        }
        let mut res = "success".to_string();
        if let Err(why) = connection().await{
            res = "fail".to_string();
            println!("{why}")
        };
        assert_eq!(&res,"success")
    }
}
