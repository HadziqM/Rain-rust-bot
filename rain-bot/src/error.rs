use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("Test")]
    Test,
}
