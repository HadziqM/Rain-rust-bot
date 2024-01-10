use poise::serenity_prelude::Colour;
use rand::{self,seq::SliceRandom};

pub struct MyColor;

impl MyColor {
    pub fn random() -> Colour {
        let mut channel = (0..0xFE01).collect::<Vec<_>>();
        channel.shuffle(&mut rand::thread_rng());
        Colour::new(channel[0]*0xFF)
    }
    pub const RED: Colour = Colour::new(0xFF0000); // 16711680 in decimal
    pub const BLUE: Colour = Colour::new(0x0000FF); // 255 in decimal
    pub const GREEN: Colour = Colour::new(0x00FF00); // 65280 in decimal
    pub const ORANGE: Colour = Colour::new(0xFF5500); // 16744256 in decimal
    pub const YELLOW: Colour = Colour::new(0xFFFF00); // 16776960 in decimal
    pub const GREY: Colour = Colour::new(0x888888); // 8888888 in decimal
    pub const BRONZE: Colour = Colour::new(0xCD7F32); // 10540642 in decimal
    pub const SILVER: Colour = Colour::new(0xC0C0C0); // 12632256 in decimal
    pub const GOLD: Colour = Colour::new(0xFFDD00); // 16766720 in decimal
}
