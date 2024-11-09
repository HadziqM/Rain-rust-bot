use common::setting::SettingAll;
use macros::Wrapper;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use thiserror::Error;

pub mod account;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("{0}")]
    Custom(String),
}

impl From<String> for DbError {
    fn from(value: String) -> Self {
        Self::Custom(value)
    }
}

impl From<&str> for DbError {
    fn from(value: &str) -> Self {
        Self::Custom(value.to_string())
    }
}

pub type DbResult<T> = Result<T, DbError>;

#[derive(Clone, Debug, Wrapper)]
pub struct Db(Pool<Postgres>);

impl Db {
    pub async fn connect(setting: &SettingAll) -> DbResult<Self> {
        let db = &setting.main.database;
        let url = format!(
            "postgres://{}:{}@{}:{}/{}",
            db.user, db.password, db.host, db.port, db.database
        );
        let pool = PgPoolOptions::new()
            .max_connections(100)
            .connect(url.as_str())
            .await?;
        Ok(Self(pool))
    }
}
