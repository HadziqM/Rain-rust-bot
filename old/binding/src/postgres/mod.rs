use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub mod account;
pub mod card;
pub mod custom;
pub mod gacha;
pub mod guild;
pub mod server;

#[derive(Debug)]
pub enum PgCustomError {
    Sqlx(sqlx::error::Error),
    Custom(String),
}

impl std::error::Error for PgCustomError {}

impl std::fmt::Display for PgCustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sqlx(x) => x.fmt(f),
            Self::Custom(x) => x.fmt(f),
        }
    }
}

impl From<sqlx::error::Error> for PgCustomError {
    fn from(value: sqlx::error::Error) -> Self {
        Self::Sqlx(value)
    }
}

impl From<&str> for PgCustomError {
    fn from(value: &str) -> Self {
        Self::Custom(value.to_owned())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DbConf {
    user: String,
    password: String,
    host: String,
    port: u16,
    database: String,
}

#[derive(Clone, Debug)]
pub struct Db(Pool<Postgres>);

impl std::ops::Deref for Db {
    type Target = Pool<Postgres>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Db {
    pub async fn connect(conf: &DbConf) -> Result<Self, PgCustomError> {
        let url = format!(
            "postgres://{}:{}@{}:{}/{}",
            conf.user, conf.password, conf.host, conf.port, conf.database
        );
        let pool = PgPoolOptions::new()
            .max_connections(100)
            .connect(url.as_str())
            .await?;
        Ok(Self(pool))
    }
}
