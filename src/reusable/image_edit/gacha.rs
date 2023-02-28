use std::{path::{Path, PathBuf}, io::Cursor};
use image::{Rgb, ImageBuffer, RgbImage};
use rusttype::Font;
use image::imageops::{FilterType,resize};
use super::CustomImageError;
use super::super::bitwise::ItemCode;
use crate::ItemPedia;
use super::Images;


#[derive(Clone)]
pub struct GachaData{
    pub result:GachaR,
    pub code:ItemCode
}
pub struct GachaImage{
    rec:Rectangle,
    font:Font<'static>,
}
#[derive(Clone,PartialEq)]
pub enum GachaR{
    R,
    SR,
    SSR,
    UR
}
pub(crate) struct Rectangle{
    pub radius:u32,
    pub off_x:u32,
    pub off_y:u32,
    pub img:ImageBuffer<Rgb<u8>,Vec<u8>>
}
impl GachaR{
    fn path(&self)->PathBuf{
        let path = Path::new(".").join("image");
        match self{
            GachaR::R=>path.join("r.jpg"),
            GachaR::SR=>path.join("sr.jpg"),
            GachaR::SSR=>path.join("ssr.jpg"),
            GachaR::UR=>path.join("ur.jpg")
        }
    }
    pub(super) fn url(&self)->String{
        let dim = (772,615);
        match self{
            GachaR::R=>format!("https://media.discordapp.net/attachments/1009291538733482055/1032987164943859712/r.jpg?width={}&height={}",dim.0,dim.1),
            GachaR::SR=>format!("https://media.discordapp.net/attachments/1009291538733482055/1032987165325537380/sr.jpg?width={}&height={}",dim.0,dim.1),
            GachaR::SSR=>format!("https://media.discordapp.net/attachments/1009291538733482055/1032987165602369566/ssr.jpg?width={}&height={}",dim.0,dim.1),
            GachaR::UR=>format!("https://media.discordapp.net/attachments/1009291538733482055/1032987165937909851/ur.jpg?width={}&height={}",dim.0,dim.1)
        }
    }
    fn raw(&self,img:&Images)->Vec<u8>{
        match self{
            GachaR::R=>img.gacha.r.to_owned(),
            GachaR::SR=>img.gacha.sr.to_owned(),
            GachaR::UR=>img.gacha.ur.to_owned(),
            GachaR::SSR=>img.gacha.ssr.to_owned()
        }
    }
}
impl Rectangle{
    pub(super) fn is_in_area(&self,x:u32,y:u32)->bool{
        //pretend center is (0,0)
        let im_x:i32= x as i32 - self.off_x as i32 - self.radius as i32;
        let im_y:i32= y as i32 - self.off_y as i32 - self.radius as i32;
        if self.radius.pow(2) as i32 >= im_x.pow(2) + im_y.pow(2){
            return true;
        }
        false
    }
    pub(super) async fn get_rect(url:&str,radius:u32,off_x:u32,off_y:u32)->Result<Rectangle,CustomImageError>{
        let client = reqwest::Client::new();
        let bytes =client.get(url).send().await?.bytes().await?;
        let read = image::io::Reader::new(Cursor::new(bytes)).with_guessed_format()?.decode()?;
        let img = read.resize_exact(radius*2+1, radius*2+1,FilterType::Nearest).to_rgb8();
        Ok(Rectangle { radius, off_x, off_y, img })
    }
    pub(super) fn get_rbg_pixel(&self,x:u32,y:u32)->Rgb<u8>{
        //normalize the pixel position
        //make center is (radius,radius)
        let norm_x = x - self.off_x;
        let norm_y = y - self.off_y;
        self.img.get_pixel(norm_x,norm_y).to_owned()
    }
}

impl GachaImage {
    pub async fn new(url:&str)->Result<GachaImage,CustomImageError>{
        let font = GachaImage::load_font().await;
        let rec = Rectangle::get_rect(url, 51, 338, 48).await?;
        Ok(GachaImage { rec, font })
    }
    async fn load_font()->Font<'static>{
        let path = Path::new(".").join("icon").join("NotoSerifJP-Regular.otf");
        let data = tokio::fs::read(&path).await;
        Font::try_from_vec(data.unwrap()).unwrap()
    }
    fn get_x(&self,text:&str)->i32{
        let len = text.len() as i32;
        386-len*16/2
    }
    async fn url_pull(&self,gacha:&GachaData,pedia:&ItemPedia,imgg:&Images)->Result<ImageBuffer<Rgb<u8>,Vec<u8>>,CustomImageError>{
        let bytes =gacha.result.raw(imgg);
        let mut img = image::io::Reader::new(Cursor::new(bytes)).with_guessed_format()?.decode()?.to_rgb8();
        //draw photo profile to image
        for (x,y,px) in img.enumerate_pixels_mut(){
            if self.rec.is_in_area(x, y){
                *px = self.rec.get_rbg_pixel(x, y)
            }
        }
        //draw text to image
        let text = match gacha.code.text(pedia){
            Some(x)=>x,
            None=>{return Err(CustomImageError::Custom("the key value on item code was false"));}
        };
        let res = imageproc::drawing::draw_text(&img,
            Rgb([255,255,255]),
            self.get_x(&text), 510,
            rusttype::Scale { x: 50.0, y: 50.0 }
            ,&self.font,&text);
        Ok(res)
    }
    pub async fn multi_pull(&self,gachas:Vec<GachaData>,pedia:&ItemPedia,imgg:&Images)->Result<Vec<u8>,CustomImageError>{
        let mut img:RgbImage = ImageBuffer::new(1028, 615);
        let mut all_buff = Vec::new();
        for i in &gachas{
            let res = &self.url_pull(i,pedia,imgg).await?;
            all_buff.push(resize(res,258,206,FilterType::Nearest))
        }
        for (x,y,px) in img.enumerate_pixels_mut(){
            let index:usize = (x as usize/257)+(y as usize/205)*4;
            let x_position = x as usize - ((index % 4)*257);
            let y_position = y as usize - ((index / 4)*205);
            *px = all_buff[index].get_pixel(x_position as u32, y_position as u32).to_owned()
        }
        let mut bytes = Vec::new();
        img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;
        Ok(bytes)
    }
    pub async fn single_pull(&self,gacha:&GachaData,pedia:&ItemPedia,imgg:&Images)->Result<Vec<u8>,CustomImageError>{
        let img = self.url_pull(gacha,pedia,imgg).await?;
        let mut bytes = Vec::new();
        img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;
        Ok(bytes)
    }
}

#[cfg(test)]
mod testing{
    // use super::*;
    //
    // #[tokio::test()]
    // async fn test_edit() {
    //     let mut cev = Vec::new();
    //     for _ in 0..12{
    //         cev.push(GachaData{result:GachaR::UR,code:ItemCode { key: "0100".to_owned() , count: 1, types:7 }})
    //     }
    //     let x = GachaImage::new("https://media.discordapp.net/attachments/998783824710344755/1023057468667998228/hello.png").await.unwrap();
    //     x.multi_pull(cev).await.unwrap();
    // }
    // #[tokio::test]
    // async fn test_single() {
    //     let cev = GachaData{result:GachaR::UR,code:ItemCode { key: "0000".to_owned() , count: 1, types:7 }};
    //     let x = GachaImage::new("https://media.discordapp.net/attachments/998783824710344755/1023057468667998228/hello.png").await.unwrap();
    //     let y = x.url_pull(&cev).await.unwrap();
    //     y.save("./test.jpg").unwrap();
    // }
}
