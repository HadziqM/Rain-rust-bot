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
    pub types:HashMap<u8,&'static str>,
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
            (0,"legs"),
            (1,"Head"),
            (2,"Chest"),
            (3,"Arms"),
            (4,"Waist"),
            (5,"Melee"),
            (6,"Ranged"),
            (7,"Item"),
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
            7=>Some(ItemList::Leg(items::Items::default().item)),
            _=>None
        }
    }
    fn get(&self,clue:&str)->Option<&'static str>{
        let val = match self{
            ItemList::Item(x)=>x,
            ItemList::Waist(x)=>x,
            ItemList::Melee(x)=>x,
            ItemList::Arms(x)=>x,
            ItemList::Head(x)=>x,
            ItemList::Ranged(x)=>x,
            ItemList::Chest(x)=>x,
            ItemList::Leg(x)=>x,
        };
        val.get(clue).copied()
    }
}
impl ItemPedia{
    pub fn search(key:u8,clue:&str)->Option<&'static str>{
        ItemList::new(key)?.get(clue)
    }
}


