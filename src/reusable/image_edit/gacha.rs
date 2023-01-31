use std::{path::{Path, PathBuf}, io::Cursor};
use image::{ImageError, Rgb, ImageBuffer};
use rusttype::Font;

pub struct GachaImage{
    rec:Rectangle,
    font:Font<'static>
}
pub enum GachaR{
    R,
    SR,
    SSR,
    UR
}
struct Rectangle{
    radius:u32,
    off_x:u32,
    off_y:u32,
    img:ImageBuffer<Rgb<u8>,Vec<u8>>
}
impl GachaR{
    fn path(&self)->PathBuf{
        let path = Path::new(".").join("gacha");
        match self{
            GachaR::R=>path.join("r.jpg"),
            GachaR::SR=>path.join("sr.jpg"),
            GachaR::SSR=>path.join("ssr.jpg"),
            GachaR::UR=>path.join("ur.jpg")
        }
    }
}
impl Rectangle{
    fn is_in_area(&self,x:u32,y:u32)->bool{
        //pretend center is (0,0)
        let im_x:i32= x as i32 - self.off_x as i32 - self.radius as i32;
        let im_y:i32= y as i32 - self.off_y as i32 - self.radius as i32;
        if self.radius.pow(2) as i32 >= im_x.pow(2) + im_y.pow(2){
            return true;
        }
        false
    }
    async fn get_rect(url:&str,radius:u32,off_x:u32,off_y:u32)->Result<Option<Rectangle>,ImageError>{
        let client = reqwest::Client::new();
        let bytes =match client.get(url).send().await{
            Ok(resp)=>{
                match resp.bytes().await{
                    Ok(byt)=>byt,
                    Err(_)=>{
                        return Ok(None);
                    }
                }
            }
            Err(_)=>{
                return Ok(None);
            }
        };
        let read = image::io::Reader::new(Cursor::new(bytes)).with_guessed_format()?.decode()?;
        let img = read.resize_exact(radius*2+1, radius*2+1, image::imageops::FilterType::Nearest).to_rgb8();
        Ok(Some(Rectangle { radius, off_x, off_y, img }))
    }
    fn get_rbg_pixel(&self,x:u32,y:u32)->Rgb<u8>{
        //normalize the pixel position
        //make center is (radius,radius)
        let norm_x = x - self.off_x;
        let norm_y = y - self.off_y;
        self.img.get_pixel(norm_x,norm_y).to_owned()
    }
}

impl GachaImage {
    pub async fn new(url:&str)->Result<GachaImage,ImageError>{
        let font = GachaImage::load_font().await;
        let rec = Rectangle::get_rect(url, 51, 338, 48).await?.unwrap();
        Ok(GachaImage { rec, font })
    }
    async fn load_font()->Font<'static>{
        let path = Path::new(".").join("icon").join("Itim-Regular.ttf");
        let data = tokio::fs::read(&path).await;
        Font::try_from_vec(data.unwrap()).unwrap()
    }
    fn get_x(&self,text:&str)->i32{
        let len = text.len() as i32;
        386-len*20/2
    }
    async fn test_image(&self,x:GachaR,text:&str)->Result<(),ImageError>{
        let mut image =  image::open(&x.path())?.to_rgb8();
        //draw photo profile to image
        for (x,y,px) in image.enumerate_pixels_mut(){
            if self.rec.is_in_area(x, y){
                *px = self.rec.get_rbg_pixel(x, y)
            }
        }
        //draw text to image
        let res = imageproc::drawing::draw_text(&image,
            Rgb([255,255,255]),
            self.get_x(text), 510,
            rusttype::Scale { x: 50.0, y: 50.0 }
            ,&self.font,text);
        res.save("test.jpg")?;
        Ok(())
    }
}

#[cfg(test)]
mod testing{
    use super::*;

    #[tokio::test()]
    async fn test_edit() {
        let x = GachaImage::new("https://media.discordapp.net/attachments/998783824710344755/1023057468667998228/hello.png").await.unwrap();
        x.test_image(GachaR::SSR,"Spirit Slash VIIx99").await.unwrap();
    }
}