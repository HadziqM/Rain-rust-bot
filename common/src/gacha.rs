use std::path::Path;

use sysdir::Sysdir;

use crate::{item_code::ItemCode, SYSDIR};

#[derive(Clone)]
pub struct GachaData {
    pub result: GachaR,
    pub code: ItemCode,
}
#[derive(Clone, PartialEq)]
pub enum GachaR {
    R,
    SR,
    SSR,
    UR,
}

impl GachaR {
    pub fn path(&self) -> Option<Sysdir> {
        let dir = Path::new("image");
        let img = match self {
            Self::SR => dir.join("sr.jpg"),
            Self::SSR => dir.join("ssr.jpg"),
            Self::R => dir.join("r.jpg"),
            Self::UR => dir.join("ur.jpg"),
        };
        SYSDIR.find_path(img)
    }
}
