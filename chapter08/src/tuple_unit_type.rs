#![allow(dead_code)]

use std::borrow::Borrow;

struct Coordinate(usize, isize);

pub fn generate_tuple() {
    let c = Coordinate(100, 200);
    println!("x = {}", c.0);
    println!("y = {}", c.1);
}

struct OneState;

pub fn generate_unit() {
    let s = OneState;
    println!("s = {:p}", s.borrow());
}

