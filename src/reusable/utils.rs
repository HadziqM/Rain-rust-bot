use serenity::utils::Colour;
use rand::{self, seq::SliceRandom};

pub enum Color{
    Red,
    Orange,
    Blue,
    Green,
    Grey,
    Random
}
impl Color {
    fn throw(&self)->Colour{
        match self{
            Self::Red=>color("ff", "00", "00"),
            Self::Blue=>color("00", "00", "ff"),
            Self::Green=>color("00", "ff", "00"),
            Self::Orange=>color("ff", "55", "00"),
            Self::Grey=>color("88", "88", "88"),
            Self::Random=>{
                let mut channel = (0..65025).collect::<Vec<_>>();
                channel.shuffle(&mut rand::thread_rng());
                Colour::new(channel[0]*255)
            }
        }
    }
}



pub fn color(red:&str,green:&str,blue:&str)-> Colour{
    let some_u32 = u32::from_str_radix(&[red,green,blue].concat(), 16);
    Colour::new(some_u32.unwrap())
}
