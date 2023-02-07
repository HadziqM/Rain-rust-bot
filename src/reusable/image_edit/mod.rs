pub mod gacha;

#[derive(Debug)]
pub enum CustomImageError{
    Custom(String)
}

impl std::fmt::Display for CustomImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomImageError::Custom(x)=>x.fmt(f)
        }
    }
}

impl std::error::Error for CustomImageError {}
