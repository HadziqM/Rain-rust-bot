use std::io::Cursor;
use image::DynamicImage;
use super::super::component::bounty::TitleImage;
use super::CustomImageError;
use super::gacha::Rectangle;

impl TitleImage {
    async fn load_image(&self)->Result<DynamicImage,CustomImageError>{
        let x = reqwest::get(&self.url).await?
            .bytes().await?.to_vec();
        Ok(image::io::Reader::new(Cursor::new(x)).with_guessed_format()?.decode()?)
    }
    async fn get_rect(&self,url:&str)->Result<Rectangle,CustomImageError>{
        Ok(Rectangle::get_rect(url, self.diameter/2, self.x_start, self.y_start).await?)
    }
    pub async fn title(&self,url:&str)->Result<Vec<u8>,CustomImageError>{
        let mut bg = self.load_image().await?.to_rgb8();
        let rec = self.get_rect(url).await?;
        for (x,y,px) in bg.enumerate_pixels_mut(){
            if rec.is_in_area(x, y){
                *px = rec.get_rbg_pixel(x, y)
            }
        }
        let mut bytes = Vec::new();
        bg.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;
        Ok(bytes)
    }
}
