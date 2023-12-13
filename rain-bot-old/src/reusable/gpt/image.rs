use super::{Gpt,MyErr};
use serde::{Serialize,Deserialize};
use serenity::builder::{CreateAttachment, EditInteractionResponse};
use crate::{Mybundle,Components};

#[derive(Serialize,Deserialize)]
struct ImageIn {
    prompt: String,
    n:u8,
    size:String,
}
#[derive(Serialize,Deserialize,Debug)]
struct Imgs {
    url:String
}
#[derive(Serialize,Deserialize,Debug)]
pub struct ImageOut {
    data: Vec<Imgs>
}

impl From<tokio::task::JoinError> for MyErr {
    fn from(value: tokio::task::JoinError) -> Self {
        MyErr::Custom(format!("failed on joining task with code:\n {value:?}"))
    }
}
async fn download(url:String)->Result<Vec<u8>,MyErr>{
    Ok(reqwest::get(url).await?.bytes().await?.to_vec())
}
impl ImageOut {
    pub async fn send<T:Mybundle>(&self,bnd:&T)->Result<(),MyErr>{
        let mut y = EditInteractionResponse::new();
        let mut set = tokio::task::JoinSet::new();
        for i in &self.data{
            set.spawn(download(i.url.to_owned()));
        }
        loop {
            match set.join_next().await{
                Some(x)=>{
                    y = y.new_attachment(CreateAttachment::bytes(x??, "image.png"));
                }
                None=>{break;}
            }
        }
        Components::edit_adv(bnd, y).await
    }
}

impl Gpt{
    async fn image(&self,data:&ImageIn)->Result<ImageOut,MyErr>{
        let url = "https://api.openai.com/v1/images/generations";
        let client =  reqwest::Client::new();
        let resp = client.post(url).headers(self.head.to_owned()).json(&data).send().await?.text().await?;
        let comp =match serde_json::from_str::<ImageOut>(&resp){
            Ok(x)=>x,
            Err(_)=>{
                return Err(MyErr::Custom(format!("getting invalid response with the parsed data:\n\n{}",&resp)));
            }
        };
        Ok(comp)
    }
    pub async fn get_image(&self,prompt:String,n:u8,size:String)->Result<ImageOut,MyErr>{
        let x = ImageIn{prompt,n,size};
        self.image(&x).await
    }
}

// #[cfg(test)]
// mod testing{
//     use super::*;
//
//     #[tokio::test]
//     async fn image() {
//         let gpt = Gpt::new().unwrap();
//         let image = gpt.get_image("monster hunter but anime".to_owned(), 2, "512x512".to_owned()).await.unwrap();
//         println!("{image:?}");
//     }
// }
