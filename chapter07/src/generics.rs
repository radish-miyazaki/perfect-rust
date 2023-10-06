#![allow(dead_code)]

use std::ops::{Sub, Add};

fn add<T: Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

fn sub<T: Sub<Output = T>>(x: T, y: T) -> T {
    x - y
}

pub fn use_add() {
    let r = add::<i32>(10, 20);
    println!("10 + 20 = {}", r);

    let r = add::<f32>(10.0, 20.0);
    println!("10.0 + 20.0 = {}", r);
}

pub fn use_sub() {
    let r = sub::<u64>(30, 20);
    println!("30 - 20 = {}", r);

    let r = sub::<f64>(30.0, 20.0);
    println!("30.0 - 20.0 = {}", r);
}
