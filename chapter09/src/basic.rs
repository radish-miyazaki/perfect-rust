#![allow(dead_code)]

use std::fmt::{Display, Formatter};

#[derive(Debug)]
#[repr(u32)]
pub enum Season {
    Spring = 100,
    Summer = 200,
    Autumn,
    Winter
}

impl Display for Season {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Spring => write!(f, "Spring(春): {}", Self::Spring as u8),
            Self::Summer => write!(f, "Summer(夏): {}", Self::Summer as u8),
            Self::Autumn => write!(f, "Autumn(秋): {}", Self::Autumn as u8),
            Self::Winter => write!(f, "Winter(冬): {}", Self::Winter as u8),
        }
    }
}

pub fn use_season() {
    let summer = Season::Summer;
    let winter = Season::Winter;
    println!("summer: {:?}", summer);
    println!("winter: {:?}", winter);

    let summer_num = Season::Summer as u8;
    let winter_num = Season::Winter as u8;
    println!("summer_num: {}", summer_num);
    println!("winter_num: {}", winter_num);
}

pub fn use_fmt() {
    println!("{}", Season::Spring);
    println!("{}", Season::Summer);
}

pub fn use_repr() {
    println!("{}", Season::Spring);
    println!("{}", Season::Summer);
    println!("{}", Season::Autumn);
    println!("{}", Season::Winter);
}
