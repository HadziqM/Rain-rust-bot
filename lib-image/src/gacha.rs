use std::{io::Cursor, sync::Arc};

use crate::{Circle, ImageResult, ImageRgb, MyImageError};
use ab_glyph::FontRef;
use common::gacha::{GachaData, GachaR};
use image::{ImageFormat, ImageReader, Rgba};
use indexmap::IndexMap;
use log::debug;
use material::ItemPedia;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct GachaCache {
    pub ur: Circle,
    pub ssr: Circle,
    pub sr: Circle,
    pub r: Circle,
    pub font: FontRef<'static>,
    pub bg: ImageRgb,
    pub ratio: f32,
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CacheKey {
    pub url: String,
    pub rarity: GachaR,
}
#[derive(Debug, Clone)]
pub struct ProcessedCache {
    pub store: IndexMap<CacheKey, Circle>,
}

#[derive(Debug, Clone)]
pub struct GachaState {
    pub raw_cache: GachaCache,
    pub processed_cache: Arc<RwLock<ProcessedCache>>,
}

impl ProcessedCache {
    /// insert cache with limit
    pub fn insert(&mut self, key: CacheKey, val: Circle) {
        if self.store.len() > 200 {
            self.store.pop();
            self.store.insert(key, val);
        }
    }
}

impl GachaState {
    pub fn new() -> ImageResult<Self> {
        let raw_cache = GachaCache::load()?;
        let processed_cache = Arc::new(RwLock::new(ProcessedCache {
            store: IndexMap::new(),
        }));

        Ok(Self {
            raw_cache,
            processed_cache,
        })
    }
}

impl GachaCache {
    pub fn load() -> ImageResult<Self> {
        let ur = Circle::load_local(GachaR::UR.bytes())?;
        let ssr = Circle::load_local(GachaR::SSR.bytes())?;
        let sr = Circle::load_local(GachaR::SR.bytes())?;
        let r = Circle::load_local(GachaR::R.bytes())?;

        let ratio = r.ratio;

        let bg = ImageReader::new(Cursor::new(include_bytes!(
            "../../image/background_upscaled.png"
        )))
        .with_guessed_format()?
        .decode()?
        .to_rgba8();

        debug!("Loaded all image binaries");

        let byte = include_bytes!("../../icon/NotoSerifJP-Regular.otf");
        let font = FontRef::try_from_slice(byte).unwrap();

        debug!("Loaded font file");

        Ok(Self {
            ur,
            ssr,
            sr,
            r,
            font,
            bg,
            ratio,
        })
    }

    fn place_background(&self, gacha_result: Vec<ImageRgb>) -> ImageRgb {
        //param
        let column = 5;

        let ratio = self.ratio;

        let padding_x_corner = (270.0 * ratio).ceil() as u32;
        let padding_x = (60.0 * ratio).ceil() as u32;
        let padding_y_up = (375.0 * ratio).ceil() as u32;
        let padding_y = (275.0 * ratio).ceil() as u32;
        let item_height = (615.0 * ratio).ceil() as u32;
        let item_width = (772.0 * ratio).ceil() as u32;
        let mut res = self.bg.clone();

        res.enumerate_pixels_mut()
            .par_bridge()
            .for_each(|(x, y, px)| {
                for (index, img) in gacha_result.iter().enumerate() {
                    let col = index % column;
                    let row_idx = index / column;

                    // Calculate image position on background
                    let img_start_x = padding_x_corner + col as u32 * (item_width + padding_x);
                    let img_end_x = img_start_x + item_width;
                    let img_start_y = padding_y_up + row_idx as u32 * (item_height + padding_y);
                    let img_end_y = img_start_y + item_height;

                    // Check if current pixel is within this image's bounds
                    if x >= img_start_x && x < img_end_x && y >= img_start_y && y < img_end_y {
                        // Calculate relative position within the image
                        let rel_x = x - img_start_x;
                        let rel_y = y - img_start_y;

                        // Get pixel from the source image
                        if let Some(source_pixel) = img.get_pixel_checked(rel_x, rel_y) {
                            *px = *source_pixel;
                        }
                        break;
                    }
                }
            });

        res
    }

    pub async fn pull(
        &self,
        res: Vec<GachaData>,
        avatar_url: impl ToString,
        pedia: &ItemPedia,
        cache: &mut ProcessedCache,
    ) -> ImageResult<Vec<u8>> {
        let url = avatar_url.to_string();
        let avatar = self.r.load_avatar(&url).await?;

        debug!("loaded and processed avatar url");

        let results = res
            .iter()
            .map(|e| -> ImageResult<(Circle, String)> {
                let key = CacheKey {
                    url: url.clone(),
                    rarity: e.result.clone(),
                };

                // get item name in database
                let item_name =
                    pedia
                        .dictionary(e.code.types, &e.code.key)
                        .ok_or(MyImageError::Custom(
                            "the key value on item code was false".to_string(),
                        ))?;
                let text = format!("{item_name} x {}", e.code.count);

                // get the processed image in cache if available
                match cache.store.get(&key) {
                    Some(img) => Ok((img.clone(), text)),
                    None => {
                        let mut cir = match e.result {
                            GachaR::SSR => self.ssr.clone(),
                            GachaR::SR => self.sr.clone(),
                            GachaR::R => self.r.clone(),
                            GachaR::UR => self.ur.clone(),
                        };

                        debug!("begin masking");
                        cir.mask_avatar(avatar.clone())?;
                        cache.insert(key, cir.clone());
                        Ok((cir, text))
                    }
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        debug!("image masked");
        let written = results
            .par_iter()
            .map(|(cir, text)| {
                let rgb = Rgba([255, 255, 255, 255]);
                let ratio = self.ratio;
                let x = (368.0 * ratio - text.len() as f32 * 8.0 * ratio).ceil() as i32;
                let y = (510.0 * ratio).ceil() as i32;
                let scale = 50.0 * ratio;

                imageproc::drawing::draw_text(&cir.img, rgb, x, y, scale, &self.font, text)
            })
            .collect::<Vec<_>>();

        debug!("all image drawed");

        let res = match written.len() == 1 {
            true => written.first().unwrap().to_owned(),
            false => self.place_background(written),
        };

        let mut byte = vec![];

        #[cfg(debug_assertions)]
        res.save("test.png")?;

        res.write_to(&mut Cursor::new(&mut byte), ImageFormat::Png)?;

        Ok(byte)

        // let horizontal_item = 5;
        // let vertical_item = res.len().div_ceil(5) as u32;
        //
        // let mut background = ImageBuffer::new(772*5, 615*vertical_item );
        //
        // background.enumerate_pixels_mut().par_bridge().for_each(|e| {
        //
        // });
    }
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, error::Error, io::Cursor, path::Path};

    use common::{
        gacha::{GachaData, GachaR},
        item_code::ItemCode,
    };
    use image::{imageops::FilterType, ImageReader};
    use indexmap::IndexMap;
    use material::ItemPedia;

    use crate::gacha::{GachaCache, ProcessedCache};

    fn downscale(path: impl AsRef<Path>, byte: Vec<u8>) -> Result<(), Box<dyn Error>> {
        let read = ImageReader::new(Cursor::new(byte))
            .with_guessed_format()?
            .decode()?;
        let resized = read.resize_to_fill(510, 408, FilterType::CatmullRom);
        resized.save(path)?;
        Ok(())
    }

    #[ignore]
    #[test]
    fn downscale_gacha() -> Result<(), Box<dyn Error>> {
        let path = Path::new("..").join("image");
        downscale(path.join("r_downscaled.jpg"), GachaR::R.bytes())?;
        downscale(path.join("sr_downscaled.jpg"), GachaR::SR.bytes())?;
        downscale(path.join("ssr_downscaled.jpg"), GachaR::SSR.bytes())?;
        downscale(path.join("ur_downscaled.jpg"), GachaR::UR.bytes())?;
        Ok(())
    }

    #[ignore]
    #[test]
    fn upscale() -> Result<(), Box<dyn Error>> {
        let bg = include_bytes!("../../image/background.png");
        let read = ImageReader::new(Cursor::new(bg))
            .with_guessed_format()?
            .decode()?;
        let resized = read.resize_to_fill(3072, 1575, FilterType::CatmullRom);
        resized.save("../image/background_upscaled.png")?;
        Ok(())
    }

    // #[ignore]
    #[tokio::test]
    async fn gacha_test() {
        logger::Mylogger::default().init();
        let raw = GachaCache::load().unwrap();
        let mut proc = ProcessedCache {
            store: IndexMap::new(),
        };
        let x = GachaData {
            result: GachaR::UR,
            code: ItemCode {
                key: "0700".to_string(),
                count: 1,
                types: 7,
            },
        };
        let data = vec![x];

        let pedia = ItemPedia::default();

        let url = "https://cdn.discordapp.com/attachments/950666821210619914/1383868770568769658/image.png?ex=68510479&is=684fb2f9&hm=87e444fdc39aba529841dba6b4b3c1abf2062e4f0be36540bdaaede76a113306&";

        raw.pull(data, url, &pedia, &mut proc).await.unwrap();
    }
}
