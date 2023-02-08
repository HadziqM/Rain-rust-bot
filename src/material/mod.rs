pub mod items;
pub mod arms;
pub mod chest;
pub mod head;
pub mod leg;
pub mod melee;
pub mod ranged;
pub mod waist;


use std::collections::HashMap;

pub struct ItemPedia{
    pub types:HashMap<u8,HashMap<&'static str,&'static str>>,
}
pub enum ItemList {
    Item(HashMap<&'static str,&'static str>),
    Arms(HashMap<&'static str,&'static str>),
    Chest(HashMap<&'static str,&'static str>),
    Head(HashMap<&'static str,&'static str>),
    Leg(HashMap<&'static str,&'static str>),
    Waist(HashMap<&'static str,&'static str>),
    Melee(HashMap<&'static str,&'static str>),
    Ranged(HashMap<&'static str,&'static str>)
}
impl Default for ItemPedia{
    fn default() -> Self {
        ItemPedia { types:HashMap::from([
            (0,leg::Leg::default().item),
            (1,head::Head::default().item),
            (2,chest::Chest::default().item),
            (3,arms::Arms::default().item),
            (4,waist::Waist::default().item),
            (5,melee::Melee::default().item),
            (6,ranged::Ranged::default().item),
            (7,items::Items::default().item),
        ])
        }
    }
}
impl ItemList{
    pub fn new(key:u8)->Option<Self>{
        match key{
            0=>Some(ItemList::Leg(leg::Leg::default().item)),
            1=>Some(ItemList::Head(head::Head::default().item)),
            2=>Some(ItemList::Chest(chest::Chest::default().item)),
            3=>Some(ItemList::Arms(arms::Arms::default().item)),
            4=>Some(ItemList::Waist(waist::Waist::default().item)),
            5=>Some(ItemList::Melee(melee::Melee::default().item)),
            6=>Some(ItemList::Ranged(ranged::Ranged::default().item)),
            7=>Some(ItemList::Item(items::Items::default().item)),
            _=>None
        }
    }
    pub fn value(&self)->HashMap<&'static str,&'static str>{
        match self{
            ItemList::Item(x)=>x.to_owned(),
            ItemList::Waist(x)=>x.to_owned(),
            ItemList::Melee(x)=>x.to_owned(),
            ItemList::Arms(x)=>x.to_owned(),
            ItemList::Head(x)=>x.to_owned(),
            ItemList::Ranged(x)=>x.to_owned(),
            ItemList::Chest(x)=>x.to_owned(),
            ItemList::Leg(x)=>x.to_owned(),
        }
    }
    fn get(&self,clue:&str)->Option<&'static str>{
        self.value().get(clue).copied()
    }
}
impl ItemPedia{
    pub fn search(key:u8,clue:&str)->Option<&'static str>{
        ItemList::new(key)?.get(clue)
    }
    pub fn dictionary(&self,key:u8,clue:&str)->Option<&'static str>{
        self.types.get(&key)?.get(clue).copied()
    }
}
#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn load_item() {
        let x = arms::Arms::default().item;
        assert_eq!(x.get("0000").unwrap().to_owned(),"No Equipment")
    }
}
