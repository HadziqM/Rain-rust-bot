pub mod items;


use std::collections::HashMap;

pub struct ItemType<'a>{
    pub item:HashMap<&'a str,&'a str>
}
impl<'a> Default for ItemType<'a>{
    fn default() -> ItemType<'a> {
        ItemType { item:HashMap::from([
            ("00","legs"),
            ("01","Head"),
            ("02","Chest"),
            ("03","Arms"),
            ("04","Waist"),
            ("05","Melee"),
            ("06","Ranged"),
            ("07","Item"),
            ("08","furniture"),
            ("09","Nothing"),
            ("0A","Null Point"),
            ("0B","Festi Point"),
            ("0C","Zeny"),
            ("0D","Null"),
            ("0E","Null Points"),
            ("0F","My Tore points"),
            ("10","Restyle Point"),
            ("11","N Points"),
            ("12","Nothing"),
            ("13","Gacha Coins"),
            ("14","Trial Gacha Coins"),
            ("15","Frontier points"),
            ("16","?"),
            ("17","Guild Points"),
            ("1E","em Box Page"),
            ("1F","Equipment Box Page "),
        ]) }
    }
}
