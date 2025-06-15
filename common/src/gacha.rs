use std::path::Path;

use sysdir::Sysdir;

use crate::{item_code::ItemCode, SYSDIR};

#[derive(Clone, Debug)]
pub struct GachaData {
    pub result: GachaR,
    pub code: ItemCode,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GachaR {
    R,
    SR,
    SSR,
    UR,
}

impl GachaR {
    pub fn bytes(&self) -> Vec<u8> {
        match self {
            Self::SR => include_bytes!("../../image/sr.jpg").to_vec(),
            Self::SSR => include_bytes!("../../image/ssr.jpg").to_vec(),
            Self::R => include_bytes!("../../image/r.jpg").to_vec(),
            Self::UR => include_bytes!("../../image/ur.jpg").to_vec(),
        }
    }
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
