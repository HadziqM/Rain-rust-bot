use std::time::SystemTime;

use rand::{self, seq::SliceRandom};
use serenity::all::Colour;

pub enum Color{
    Red,
    Orange,
    Blue,
    Green,
    Grey,
    Yellow,
    Random,
    Gold,
    Silver,
    Bronze,
}

pub struct MyTime;

impl MyTime {
    pub fn now()-> i64{
        i64::try_from(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()).unwrap()
    }
    pub fn elapsed(el:i64)-> i64{
        MyTime::now() + el
    }
}

impl Color {
    pub fn throw(&self)->Colour{
        match self{
            Self::Red=>color("ff", "00", "00"),
            Self::Blue=>color("00", "00", "ff"),
            Self::Green=>color("00", "ff", "00"),
            Self::Orange=>color("ff", "55", "00"),
            Self::Yellow=>color("ff", "ff", "00"),
            Self::Grey=>color("88", "88", "88"),
            Self::Bronze=>color("cd", "7f", "32"),
            Self::Silver=>color("c0", "c0", "c0"),
            Self::Gold=>color("ff", "dd", "00"),
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
