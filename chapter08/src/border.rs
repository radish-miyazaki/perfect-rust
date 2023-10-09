#![allow(dead_code)]

use std::ops::Add;
use std::marker::Copy;

#[derive(Debug, Clone)]
struct Customer<T> {
    id: T,
    name: String,
    address: String,
    email: String
}

impl<T> Customer<T> where T: Add + Copy {
    fn change_id(&mut self, val: T) {
        self.id = val;
    }
}
