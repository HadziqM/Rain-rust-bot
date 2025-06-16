use std::{io::Cursor, path::Path};

use image::{imageops::FilterType, DynamicImage, ImageBuffer, ImageFormat, ImageReader, Rgb, Rgba};
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
pub type ImageRgb = ImageBuffer<Rgba<u8>, Vec<u8>>;

/// Rounded Profile picture mask
#[derive(Debug, Clone)]
pub struct Circle {
    pub radius: u32,
    pub off_x: u32,
    pub off_y: u32,
    pub ratio: f32,
    pub img: ImageRgb,
}

impl Circle {
    pub fn load_local(byte: Vec<u8>) -> ImageResult<Self> {
        let img = ImageReader::new(Cursor::new(byte))
            .with_guessed_format()?
            .decode()?
            .to_rgba8();

        let ratio = img.width() as f32 / 772.0;
        let radius = (51.0 * ratio).ceil() as u32;
        let off_x = (335.0 * ratio).ceil() as u32;
        let off_y = (48.0 * ratio).ceil() as u32;

        Ok(Self {
            img,
            radius,
            off_x,
            off_y,
            ratio,
        })
    }

    pub async fn load_avatar(&self, url: impl ToString) -> ImageResult<ImageRgb> {
        let client = reqwest::Client::new();
        let bytes = client.get(url.to_string()).send().await?.bytes().await?;

        let read = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()?
            .decode()?;

        Ok(read
            .resize_exact(
                self.radius * 2 + 1,
                self.radius * 2 + 1,
                FilterType::CatmullRom,
            )
            .to_rgba8())
    }

    /// need to use avatar with exact size as radius
    pub fn mask_avatar(&mut self, avatar: ImageRgb) -> ImageResult<()> {
        self.img
            .enumerate_pixels_mut()
            .par_bridge()
            .for_each(|(x, y, px)| {
                // Transform image coordinates to avatar coordinates
                let avatar_x = x as i32 - self.off_x as i32;
                let avatar_y = y as i32 - self.off_y as i32;

                // Transform to circle-centered coordinates
                let dx = avatar_x - self.radius as i32;
                let dy = avatar_y - self.radius as i32;

                let avatar_width = avatar.width();
                let avatar_height = avatar.height();

                // Check if inside circle AND within avatar bounds
                if (dx * dx + dy * dy) <= (self.radius as i32).pow(2)
                    && avatar_x >= 0
                    && avatar_y >= 0
                    && (avatar_x as u32) < avatar_width
                    && (avatar_y as u32) < avatar_height
                {
                    log::debug!("pixel replaced at coordinate ({x},{y})");
                    // Replace pixel with avatar pixel
                    *px = *avatar.get_pixel(avatar_x as u32, avatar_y as u32);
                }
            });
        Ok(())
    }
}
