pub mod items;
pub mod arms;
pub mod chest;
pub mod head;
pub mod leg;
pub mod melee;
pub mod ranged;
pub mod waist;
pub mod items_jp;
pub mod arms_jp;
pub mod chest_jp;
pub mod head_jp;
pub mod leg_jp;
pub mod melee_jp;
pub mod ranged_jp;
pub mod waist_jp;


use std::collections::HashMap;

#[derive(Debug,Clone)]
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
            (0,ItemPedia::matching(leg::Leg::default().item, leg_jp::Leg::default().item)),
            (1,ItemPedia::matching(head::Head::default().item,head_jp::Head::default().item)),
            (2,ItemPedia::matching(chest::Chest::default().item,chest_jp::Chest::default().item)),
            (3,ItemPedia::matching(arms::Arms::default().item,arms_jp::Arms::default().item)),
            (4,ItemPedia::matching(waist::Waist::default().item,waist_jp::Waist::default().item)),
            (5,ItemPedia::matching(melee::Melee::default().item,melee_jp::Melee::default().item)),
            (6,ItemPedia::matching(ranged::Ranged::default().item,ranged_jp::Ranged::default().item)),
            (7,ItemPedia::matching(items::Items::default().item,items_jp::Items::default().item)),
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
    fn matching<'a>(en:HashMap<&'a str,&'a str>,jp:HashMap<&'a str,&'a str>)->HashMap<&'a str,&'a str>{
        let mut out = HashMap::new();
        for (k,v) in en{
            if v.len() == 0{
                let val = match jp.get(k){
                    Some(x)=>x.to_owned(),
                    None=>"No name"
                };
                out.insert(k,val);
            }else {
                out.insert(k, v);
            }
        }
        out
    }
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
        let pedia = ItemPedia::default();
        assert_eq!(pedia.dictionary(7, "4404"),Some("父への憧憬"))
    }
}
