use std::collections::HashMap;

use crate::{Circle, ImageResult, MyImageError};
use ab_glyph::FontRef;
use common::gacha::{GachaData, GachaR};
use image::Rgba;
use material::ItemPedia;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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

impl GachaCache {
    pub fn load() -> ImageResult<Self> {
        let ur = Circle::load_local(GachaR::UR.bytes(), 51, 338, 48)?;
        let ssr = Circle::load_local(GachaR::SSR.bytes(), 51, 338, 48)?;
        let sr = Circle::load_local(GachaR::SR.bytes(), 51, 338, 48)?;
        let r = Circle::load_local(GachaR::R.bytes(), 51, 338, 48)?;

        let byte = include_bytes!("../../icon/NotoSerifJP-Regular.otf");
        let font = FontRef::try_from_slice(byte).unwrap();

        Ok(Self {
            ur,
            ssr,
            sr,
            r,
            font,
        })
    }

    pub async fn multi_pull(
        &self,
        res: Vec<GachaData>,
        avatar_url: impl ToString,
        pedia: &ItemPedia,
        cache: &mut ProcessedCache,
    ) -> ImageResult<()> {
        let url = avatar_url.to_string();
        let avatar = self.r.load_avatar(&url).await?;

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
                        cir.mask_avatar(avatar.clone())?;
                        cache.store.insert(key, cir.clone());
                        Ok((cir, text, x))
                    }
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        let written = results.par_iter().map(|(cir, text, x)| {
            let rgb = Rgba([255, 255, 255, 255]);
            imageproc::drawing::draw_text(&cir.img, rgb, *x as i32, 510, 50.0, &self.font, text)
        });
        todo!()
    }
}
