use super::{Bitwise,BitwiseError};
use std::collections::HashMap;


impl Bitwise{
    fn reverse_key(key:&str)->Result<String, BitwiseError>{
        if key.len()!=4{
            Err(BitwiseError::InvalidKey)
        }else {
            let key:Vec<_> = key.chars().collect();
            Ok([key[2],key[3],key[0],key[1]].iter().collect::<String>())
        }
    }
    pub fn single_item(key:&str,count:u16)->Result<Vec<u8>,BitwiseError>{
        let reversed = Bitwise::reverse_key(key)?;
        let data = format!("0001070000{}0000{:04X}00000000",&reversed,count);
        Bitwise::decode(&data)
    }
    pub fn multiple_item(map:HashMap<&str,u16>)->Result<Vec<u8>,BitwiseError>{
        let mut data = format!("{:04x}",map.len());
        for (key,value) in map{
            data.push_str(&format!("070000{}0000{:04X}00000000",Bitwise::reverse_key(key)?,value));
        }
        Bitwise::decode(&data)
    }
}
