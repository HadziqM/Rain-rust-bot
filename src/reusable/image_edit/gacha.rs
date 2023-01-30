use std::{path::Path, io::Cursor};
use image::{ImageError, Rgb, ImageBuffer};


pub struct GachaImage;
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
    async fn open_image(x:GachaR)->Result<(),ImageError>{
        let path = Path::new(".").join("gacha");
        let gac = match x{
            GachaR::R=>path.join("r.jpg"),
            GachaR::SR=>path.join("sr.jpg"),
            GachaR::SSR=>path.join("ssr.jpg"),
            GachaR::UR=>path.join("ur.jpg")
        };
        let mut image =  image::io::Reader::open(&gac)?.decode()?.to_rgb8();
        let rec = Rectangle::get_rect("https://media.discordapp.net/attachments/998783824710344755/1023057468667998228/hello.png", 51, 338, 48).await?.unwrap();
        for (x,y,px) in image.enumerate_pixels_mut(){
            if rec.is_in_area(x, y){
                *px = rec.get_rbg_pixel(x, y)
            }
        }        
        image.save("idk2.png")?;
        Ok(())
    }
}

#[cfg(test)]
mod testing{
    use super::*;

    #[tokio::test()]
    async fn test_edit() {
        GachaImage::open_image(GachaR::UR).await.unwrap();
    }
}
