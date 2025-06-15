use std::{collections::HashMap, io::Cursor, sync::Arc};

use crate::{Circle, ImageResult, MyImageError};
use ab_glyph::FontRef;
use common::gacha::{GachaData, GachaR};
use image::{ImageBuffer, ImageFormat, Rgb};
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
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CacheKey {
    pub url: String,
    pub rarity: GachaR,
}
#[derive(Debug, Clone)]
pub struct ProcessedCache {
    pub store: HashMap<CacheKey, Circle>,
}

#[derive(Debug, Clone)]
pub struct GachaState {
    pub raw_cache: GachaCache,
    pub processed_cache: Arc<RwLock<ProcessedCache>>,
}

impl GachaState {
    pub fn new() -> ImageResult<Self> {
        let raw_cache = GachaCache::load()?;
        let processed_cache = Arc::new(RwLock::new(ProcessedCache {
            store: HashMap::new(),
        }));
        Ok(Self {
            raw_cache,
            processed_cache,
        })
    }
}

impl GachaCache {
    pub fn load() -> ImageResult<Self> {
        let ur = Circle::load_local(GachaR::UR.bytes(), 51, 338, 48)?;
        let ssr = Circle::load_local(GachaR::SSR.bytes(), 51, 338, 48)?;
        let sr = Circle::load_local(GachaR::SR.bytes(), 51, 338, 48)?;
        let r = Circle::load_local(GachaR::R.bytes(), 51, 338, 48)?;

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
        })
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
            .map(|e| -> ImageResult<(Circle, String, usize)> {
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

                let x = 368 - text.len() * 16 / 2;

                // get the processed image in cache if available
                match cache.store.get(&key) {
                    Some(img) => Ok((img.clone(), text, x)),
                    None => {
                        let mut cir = match e.result {
                            GachaR::SSR => self.ssr.clone(),
                            GachaR::SR => self.sr.clone(),
                            GachaR::R => self.r.clone(),
                            GachaR::UR => self.ur.clone(),
                        };

                        debug!("begin masking");
                        cir.mask_avatar(avatar.clone())?;
                        cache.store.insert(key, cir.clone());
                        Ok((cir, text, x))
                    }
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        debug!("image masked");
        let written = results
            .par_iter()
            .map(|(cir, text, x)| {
                let rgb = Rgb([255, 255, 255]);
                imageproc::drawing::draw_text(&cir.img, rgb, *x as i32, 510, 50.0, &self.font, text)
            })
            .collect::<Vec<_>>();

        let mut byte = vec![];

        #[cfg(debug_assertions)]
        written.first().unwrap().save("test.jpg")?;

        written
            .first()
            .unwrap()
            .write_to(&mut Cursor::new(&mut byte), ImageFormat::Jpeg)?;

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
    use std::collections::HashMap;

    use common::{
        gacha::{GachaData, GachaR},
        item_code::ItemCode,
    };
    use material::ItemPedia;

    use crate::gacha::{GachaCache, ProcessedCache};

    #[tokio::test]
    async fn gacha_test() {
        logger::Mylogger::default().init();
        let raw = GachaCache::load().unwrap();
        let mut proc = ProcessedCache {
            store: HashMap::new(),
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

        raw.pull(data, "https://media.discordapp.net/attachments/950666821210619914/1383712831505039431/image-1.png?ex=684fca7f&is=684e78ff&hm=0de9c1af13eca72781d7bdd3ccb471a5241201b768dd16a138d61bbcddf21563&=&format=webp&quality=lossless", &pedia, &mut proc).await.unwrap();
    }
}
