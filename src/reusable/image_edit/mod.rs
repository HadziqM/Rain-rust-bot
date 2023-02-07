pub mod gacha;

#[derive(Debug)]
pub enum CustomImageError{
    Custom(&'static str),
    Tokio(tokio::io::Error),
    Image(image::ImageError)
}

impl std::fmt::Display for CustomImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomImageError::Custom(x)=>x.fmt(f),
            CustomImageError::Tokio(x)=>x.fmt(f),
            CustomImageError::Image(x)=>x.fmt(f)
        }
    }
}

impl std::error::Error for CustomImageError {}

impl From<tokio::io::Error> for CustomImageError {
    fn from(value: tokio::io::Error) -> Self {
        CustomImageError::Tokio(value)
    }
}
impl From<image::ImageError> for CustomImageError {
    fn from(value: image::ImageError) -> Self {
        CustomImageError::Image(value)
    }
}
