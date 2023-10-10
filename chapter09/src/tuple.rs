#![allow(dead_code)]

pub enum Season<'a> {
    Spring(u8, Vec<&'a str>),
    Summer(u8, Vec<&'a str>),
    Autumn(u8, Vec<&'a str>),
    Winter(u8, Vec<&'a str>),
}

impl<'a> Season<'a> {
    pub fn format_variant(&self) -> String {
        match self {
            Self::Spring(x, y) => format!("Spring: {} months {:?}", x, y),
            Self::Summer(x, y) => format!("Summer: {} months {:?}", x, y),
            Self::Autumn(x, y) => format!("Autumn: {} months {:?}", x, y),
            Self::Winter(x, y) => format!("Winter: {} months {:?}", x, y),
        }
    }
}

pub fn use_tuple() {
    let spring = Season::Spring(3, vec!["March", "April", "May"]);
    let summer = Season::Summer(3, vec!["June", "July", "August"]);
    let autumn = Season::Autumn(3, vec!["September", "October", "November"]);
    let winter = Season::Winter(3, vec!["December", "January", "February"]);

    println!("{}", spring.format_variant());
    println!("{}", summer.format_variant());
    println!("{}", autumn.format_variant());
    println!("{}", winter.format_variant());
}
