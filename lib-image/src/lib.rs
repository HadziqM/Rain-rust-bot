use std::{io::Cursor, path::Path};

use common::gacha::GachaR;
use image::{imageops::FilterType, DynamicImage, ImageBuffer, ImageReader, Rgb};
use rayon::prelude::*;
use thiserror::Error;

pub mod gacha;

#[derive(Debug, Error)]
pub enum MyImageError {
    #[error("image error {0}")]
    Image(#[from] image::ImageError),
    #[error("reqwest error {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Tokio error {0}")]
    Tokio(#[from] tokio::io::Error),
    #[error("Custom error {0}")]
    Custom(String),
}
pub type ImageResult<T> = Result<T, MyImageError>;

/// Rounded Profile picture mask
#[derive(Debug, Clone)]
pub struct Circle {
    pub radius: u32,
    pub off_x: u32,
    pub off_y: u32,
    pub img: DynamicImage,
}
#[derive(Debug, Clone)]
pub struct GachaCache {
    pub ur: Circle,
    pub ssr: Circle,
    pub sr: Circle,
    pub r: Circle,
}

impl GachaCache {
    pub fn load() -> ImageResult<Self> {
        let ur = Circle::load_local(GachaR::UR.path().unwrap(), 51, 338, 48)?;
        let ssr = Circle::load_local(GachaR::SSR.path().unwrap(), 51, 338, 48)?;
        let sr = Circle::load_local(GachaR::SR.path().unwrap(), 51, 338, 48)?;
        let r = Circle::load_local(GachaR::R.path().unwrap(), 51, 338, 48)?;
        Ok(Self { ur, ssr, sr, r })
    }
}

impl Circle {
    pub fn load_local(
        path: impl AsRef<Path>,
        radius: u32,
        off_x: u32,
        off_y: u32,
    ) -> ImageResult<Self> {
        let img = ImageReader::open(path)?.decode()?;
        Ok(Self {
            img,
            radius,
            off_x,
            off_y,
        })
    }

    pub async fn mask_avatar(
        &self,
        url: impl ToString,
    ) -> ImageResult<ImageBuffer<Rgb<u8>, Vec<u8>>> {
        let client = reqwest::Client::new();
        let bytes = client.get(url.to_string()).send().await?.bytes().await?;
        let read = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()?
            .decode()?;
        let img = read
            .resize_exact(
                self.radius * 2 + 1,
                self.radius * 2 + 1,
                FilterType::Nearest,
            )
            .to_rgb8();

        let mut bg = self.img.clone().to_rgb8();

        bg.enumerate_pixels_mut()
            .par_bridge()
            .for_each(|(x, y, px)| {
                // move pixel to no offside
                let corner_x = x as i32 - self.off_x as i32;
                let corner_y = y as i32 - self.off_y as i32;

                // move center to (0,0)
                let dx = corner_x - self.radius as i32;
                let dy = corner_x - self.radius as i32;

                // vector equation √(x² + y²) = r
                if (dx * dx + dy * dy) <= (self.radius as i32).pow(2) {
                    // replace pixel with avatar pixel
                    *px = img.get_pixel(corner_x as u32, corner_y as u32).to_owned();
                }
            });
        Ok(bg)
    }
}
