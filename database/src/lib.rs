use common::{setting::SettingAll, SYSDIR};
use log::{debug, info};
use macros::Wrapper;
use sqlx::{
    migrate::MigrateDatabase, postgres::PgPoolOptions, sqlite::SqlitePoolOptions, Pool, Postgres,
    Sqlite,
};
use thiserror::Error;

pub mod account;
pub mod card;
pub mod distribution;
pub mod query;

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
pub struct DbLite(Pool<Sqlite>);

impl DbLite {
    pub async fn connect() -> DbResult<Self> {
        let path = SYSDIR.config_dir("bot.db").execute_dir();
        debug!("{path:?}");
        let url = format!("sqlite:{}", path.display());
        let exist = Sqlite::database_exists(&url).await?;
        if !exist {
            Sqlite::create_database(&url).await?;
        }
        let pool = SqlitePoolOptions::new()
            .max_connections(100)
            .connect(&url)
            .await?;
        if !exist {
            info!("Initialize New Sqlite Database with url: {url}");
            let scheme = include_str!("../../query/table1.sql");
            sqlx::raw_sql(scheme).execute(&pool).await?;
        }
        info!("Sqlite Database Ready to use");
        Ok(Self(pool))
    }
}

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
        info!("Server Database Connected");
        Ok(Self(pool))
    }
}

#[cfg(test)]
mod test {
    use crate::DbLite;

    #[tokio::test]
    async fn sqlite() {
        logger::Mylogger::default().init();

        DbLite::connect().await.unwrap();
    }
}
