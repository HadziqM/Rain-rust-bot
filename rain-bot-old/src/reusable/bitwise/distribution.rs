use super::{Bitwise, BitwiseError, ItemCode};
use crate::material::ItemPedia;

impl ItemCode {
    pub fn reverse_key(&self) -> Result<String, BitwiseError> {
        if self.key.len() != 4 {
            Err(BitwiseError::InvalidKey)
        } else {
            let key: Vec<_> = self.key.chars().collect();
            Ok([key[2], key[3], key[0], key[1]].iter().collect::<String>())
        }
    }
    pub fn text(&self, item: &ItemPedia) -> Option<String> {
        let item = item.dictionary(self.types, &self.key)?;
        Some(format!("{} x{}", item, self.count))
    }
    pub fn check(&self, item: &ItemPedia) -> bool {
        item.dictionary(self.types, &self.key).is_some()
    }
}
impl<'a> Default for Bitwise<'a> {
    fn default() -> Bitwise<'a> {
        Bitwise { item: &[] }
    }
}
impl<'a> Bitwise<'a> {
    pub fn new(data: &'a [ItemCode]) -> Bitwise<'a> {
        Bitwise { item: data }
    }
    fn no_item(&self) -> Result<(), BitwiseError> {
        if self.item.len() == 0 {
            Err(BitwiseError::NoItem)
        } else {
            Ok(())
        }
    }
    pub fn multiple_item(&self) -> Result<Vec<u8>, BitwiseError> {
        self.no_item()?;
        let mut data = format!("{:04X}", self.item.len());
        for code in self.item {
            data.push_str(&format!(
                "{:02X}0000{}0000{:04X}00000000",
                code.types,
                code.reverse_key()?,
                code.count
            ));
        }
        Bitwise::decode(&data)
    }
}
