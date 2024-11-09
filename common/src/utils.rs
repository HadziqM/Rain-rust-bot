use std::path::PathBuf;

use crate::SYSDIR;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UserType {
    Admin,
    Registered,
    Unregistered,
}

pub enum AssetIcon {
    Gs,
    HS,
    H,
    L,
    SS,
    LB,
    DS,
    LS,
    HH,
    GL,
    B,
    T,
    SAF,
    MS,
}

impl AssetIcon {
    pub fn local_path(&self) -> PathBuf {
        let asset = SYSDIR.assets_dir("icon").execute_dir();
        match self {
            Self::Gs => asset.join("GS.png"),
            Self::HS => asset.join("HS.png"),
            Self::H => asset.join("H.png"),
            Self::L => asset.join("L.png"),
            Self::SS => asset.join("SS.png"),
            Self::LB => asset.join("LB.png"),
            Self::DS => asset.join("DS.png"),
            Self::LS => asset.join("LS.png"),
            Self::HH => asset.join("HH.png"),
            Self::GL => asset.join("GL.png"),
            Self::B => asset.join("B.png"),
            Self::T => asset.join("T.png"),
            Self::SAF => asset.join("SAF.png"),
            Self::MS => asset.join("MS.png"),
        }
    }
    pub fn from_i32(n: i32) -> Self {
        match n {
            0 => Self::Gs,
            1 => Self::HS,
            2 => Self::H,
            3 => Self::L,
            4 => Self::SS,
            5 => Self::LB,
            6 => Self::DS,
            7 => Self::LS,
            8 => Self::HH,
            9 => Self::GL,
            10 => Self::B,
            11 => Self::T,
            12 => Self::SAF,
            13 => Self::MS,
            _ => Self::Gs,
        }
    }
}
