use binding::postgres::PgCustomError;

#[derive(Debug)]
pub enum MyErr{
    Serenity(serenity::Error),
    Tokio(tokio::io::Error),
    Utf8(std::str::Utf8Error),
    Serde(serde_json::Error),
    ByteWise(binding::bitwise::BitwiseError),
    Image(image_edit::CustomImageError),
    Custom(String)
}
impl std::error::Error for MyErr{}
impl std::fmt::Display for MyErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            MyErr::Tokio(x)=>x.fmt(f),
            MyErr::Serenity(x)=>x.fmt(f),
            MyErr::Utf8(x)=>x.fmt(f),
            MyErr::Serde(x)=>x.fmt(f),
            MyErr::Custom(x)=>x.fmt(f),
            MyErr::ByteWise(x)=>x.fmt(f),
            MyErr::Image(x)=>x.fmt(f)
        }
    }
}
impl From<binding::bitwise::BitwiseError> for MyErr {
    fn from(value: binding::bitwise::BitwiseError) -> Self {
        MyErr::ByteWise(value)
    }
}
impl From<tokio::io::Error> for MyErr{
    fn from(value: tokio::io::Error) -> Self {
        MyErr::Tokio(value)
    }
}
impl From<serenity::Error> for MyErr {
    fn from(value: serenity::Error) -> Self {
        MyErr::Serenity(value)
    }
}
impl From<std::str::Utf8Error> for MyErr{
    fn from(value: std::str::Utf8Error) -> Self {
        MyErr::Utf8(value)
    }
}
impl From<serde_json::Error> for MyErr {
    fn from(value: serde_json::Error) -> Self {
        MyErr::Serde(value)
    }
}
impl From<image_edit::CustomImageError> for MyErr {
    fn from(value: image_edit::CustomImageError) -> Self {
        MyErr::Image(value)
    }
}
impl From<&str> for MyErr {
    fn from(value: &str) -> Self {
        MyErr::Custom(value.to_string())
    }
}
impl From<binding::postgres::PgCustomError> for MyErr {
    fn from(value: binding::postgres::PgCustomError) -> Self {
        match value {
            PgCustomError::Sqlx(x) => MyErr::ByteWise(x.into()),
            PgCustomError::Custom(x) => MyErr::Custom(x)
        }
    }
}


