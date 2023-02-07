use super::{Bitwise,BitwiseError,ItemCode};
use crate::material::ItemPedia;


impl ItemCode{
    fn reverse_key(&self)->Result<String, BitwiseError>{
        if self.key.len()!=4{
            Err(BitwiseError::InvalidKey)
        }else {
            let key:Vec<_> = self.key.chars().collect();
            Ok([key[2],key[3],key[0],key[1]].iter().collect::<String>())
        }
    }
    pub fn text(&self)->Option<String>{
        let item = ItemPedia::search(self.types, &self.key)?;
        Some(format!("{}x{}",item,self.count))
    }
} 
impl<'a> Default for Bitwise<'a> {
    fn default() -> Bitwise<'a> {
        Bitwise { item: &[] }
    }
}
impl<'a> Bitwise<'a> {
    pub fn new(data:&'a [ItemCode])->Bitwise<'a>{
        Bitwise { item: data }
    }
    pub fn first_item(&self)->Result<Vec<u8>,BitwiseError>{
        if let Some(data) = self.item.first(){
            let reversed = data.reverse_key()?;
            let data = format!("0001{:02X}0000{}0000{:04X}00000000",data.types,&reversed,data.count);
            Bitwise::decode(&data)
        }else{
            Err(BitwiseError::NoItem)
        }
    }
    pub fn multiple_item(&self)->Result<Vec<u8>,BitwiseError>{
        if self.item.len() == 0{
            return Err(BitwiseError::NoItem);
        }
        let mut data = format!("{:04x}",self.item.len());
        for code in self.item{
            data.push_str(&format!("{:02X}0000{}0000{:04X}00000000",
                code.types,code.reverse_key()?,code.count));
        }
        Bitwise::decode(&data)
    }
}
