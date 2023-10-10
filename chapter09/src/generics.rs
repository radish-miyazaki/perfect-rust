#![allow(dead_code)]

use std::iter::IntoIterator;

#[derive(Debug, Clone)]
pub enum Season<T> {
    Spring(u8, T),
    Summer(u8, T),
    Autumn(u8, T),
    Winter(u8, T),
}

impl<T> Season<T> {
    pub fn get_months(&self) -> &T where T: IntoIterator {
        match self {
            Self::Spring(_, months) => months,
            Self::Summer(_, months) => months,
            Self::Autumn(_, months) => months,
            Self::Winter(_, months) => months,
        }
    }
}

pub fn use_generics() {
    use std::collections::LinkedList;

    let spring = Season::Spring(3, vec!["March", "April", "May"]);
    println!("Spring: {:?}", spring.get_months());

    let summer = Season::Summer(3, ["June", "July", "August"]);
    println!("Summer: {:?}", summer.get_months());

    let autumn = Season::Autumn(3, LinkedList::from(["September", "October", "November"]));
    println!("Autumn: {:?}", autumn.get_months());
}
