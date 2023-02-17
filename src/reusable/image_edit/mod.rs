pub mod gacha;

#[derive(Debug,Clone)]
pub struct Images {
    pub gacha: Gacha
}
#[derive(Debug,Clone)]
pub struct Gacha {
    pub ur: Vec<u8>,
    pub ssr: Vec<u8>,
    pub sr:Vec<u8>,
    pub r:Vec<u8>
}
impl Gacha{
    async fn new()->Result<Gacha,CustomImageError>{
        use gacha::GachaR;
        let ur = reqwest::get(&GachaR::UR.url()).await?.bytes().await?.to_vec();
        let ssr = reqwest::get(&GachaR::SSR.url()).await?.bytes().await?.to_vec();
        let sr = reqwest::get(&GachaR::SR.url()).await?.bytes().await?.to_vec();
        let r = reqwest::get(&GachaR::R.url()).await?.bytes().await?.to_vec();
        Ok(Gacha { ur, ssr, sr, r })
    }
}
impl Images {
    pub async fn new()->Result<Images,CustomImageError>{
        let gacha = Gacha::new().await?;
        Ok(Images { gacha })
    }
}
#[derive(Debug)]
pub enum CustomImageError{
    Custom(&'static str),
    Tokio(tokio::io::Error),
    Image(image::ImageError),
    Reqwest(reqwest::Error)
}

impl std::fmt::Display for CustomImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomImageError::Custom(x)=>x.fmt(f),
            CustomImageError::Tokio(x)=>x.fmt(f),
            CustomImageError::Image(x)=>x.fmt(f),
            CustomImageError::Reqwest(x)=>x.fmt(f),
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
impl From<reqwest::Error> for CustomImageError{
    fn from(value: reqwest::Error) -> Self {
        CustomImageError::Reqwest(value)
    }
}
