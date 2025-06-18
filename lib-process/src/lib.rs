use std::sync::Arc;

use common::{
    database::{formatted::FormattedUserData, raw::DbUserData},
    setting::SettingAll,
};
use database::{Db, DbError};
use indexmap::IndexMap;
use lib_image::gacha::GachaState;
use thiserror::Error;
use tokio::sync::RwLock;

pub mod gacha;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("{0}")]
    Custom(String),
    #[error("sqlx error: {0}")]
    Db(DbError),
    #[error("Tokio IO error: {0}")]
    Tokio(#[from] tokio::io::Error),
}

type MyResult<T> = Result<T, MyError>;

impl From<&str> for MyError {
    fn from(err: &str) -> Self {
        MyError::Custom(err.to_string())
    }
}

impl From<DbError> for MyError {
    fn from(err: DbError) -> Self {
        match err {
            DbError::Sqlx(_) => Self::Db(err),
            DbError::Custom(str) => Self::Custom(str),
        }
    }
}

pub struct UserCache {
    pub store: IndexMap<String, FormattedUserData>,
}

impl UserCache {
    /// insert cache with limit
    pub fn insert(&mut self, key: impl ToString, val: FormattedUserData) {
        if self.store.len() >= 2000 {
            self.store.pop();
            self.store.insert(key.to_string(), val);
        }
    }
    pub fn new() -> Self {
        Self {
            store: IndexMap::with_capacity(2000),
        }
    }
}

#[derive(Clone)]
pub struct App {
    pub setting: Arc<RwLock<SettingAll>>,
    pub db: Db,
    pub pedia: Arc<material::ItemPedia>,
    pub gacha: Arc<GachaState>,
    pub user_cache: Arc<RwLock<UserCache>>,
}

pub enum RegisteredStatus {
    FullyRegistered { user: FormattedUserData },
    PartiallyRegistered { user: DbUserData },
    Unregistered,
}

/// did is discord ID
impl App {
    pub async fn new() -> Self {
        let setting = SettingAll::load_all();
        App {
            db: Db::connect(&setting).await.unwrap(),
            setting: Arc::new(RwLock::new(setting)),
            pedia: Arc::new(material::ItemPedia::default()),
            gacha: Arc::new(GachaState::new().unwrap()),
            user_cache: Arc::new(RwLock::new(UserCache::new())),
        }
    }

    // implement cached user
    pub async fn get_user_status(&self, did: impl ToString) -> RegisteredStatus {
        let did = did.to_string();
        let read_cache = self.user_cache.read().await;
        match read_cache.store.get(&did) {
            Some(data) => RegisteredStatus::FullyRegistered {
                user: data.to_owned(),
            },
            None => match self.db.fetch_user_data(&did).await {
                Ok(data) => match FormattedUserData::check_if_full(&data) {
                    Ok(user) => {
                        let mut edit_cache = self.user_cache.write().await;
                        edit_cache.insert(&did, user.clone());
                        RegisteredStatus::FullyRegistered { user }
                    }
                    Err(_) => RegisteredStatus::PartiallyRegistered { user: data },
                },
                Err(_) => RegisteredStatus::Unregistered,
            },
        }
    }

    pub async fn only_register_user(&self, did: impl ToString) -> MyResult<FormattedUserData> {
        match self.get_user_status(did).await {
            RegisteredStatus::FullyRegistered { user } => Ok(user),
            RegisteredStatus::PartiallyRegistered { user:_ } => Err(MyError::Custom("User isnt fully registered yet, please use `/switch` to select your main character".to_string())),
            RegisteredStatus::Unregistered => {
                Err(MyError::Custom("User not registered please use `/bind` to binde existing game account or `/create` to create new game account".to_string()))
            }
        }
    }
    pub async fn only_unregister_user(&self, did: impl ToString) -> MyResult<()> {
        match self.get_user_status(did).await {
            RegisteredStatus::FullyRegistered { user:_ } => Err(MyError::Custom("User is already fully registered with".to_string())),
            RegisteredStatus::PartiallyRegistered { user:_ } => Err(MyError::Custom("User isnt fully registered yet, please use `/switch` to select your main character".to_string())),
            RegisteredStatus::Unregistered => Ok(())
        }
    }
}
