#[allow(dead_code)]
pub fn symbol() {
    let mut result = 10 + 20;
    println!("10 + 20 = {}", result);
    result = 20 - 10;
    println!("20 - 10 = {}", result);
    result = 20 * 10;
    println!("20 * 10 = {}", result);
    result = 20 / 10;
    println!("20 / 10 = {}", result);
    result = 10 % 3;
    println!("10 % 3 = {}", result);
}

#[allow(dead_code)]
pub fn methods(x: i32, y: i32) {
    use std::ops::{Add, Div, Mul, Rem, Sub};
    println!("{} + {} = {}", x, y, x.add(y));
    println!("{} - {} = {}", x, y, x.sub(y));
    println!("{} * {} = {}", x, y, x.mul(y));
    println!("{} / {} = {}", x, y, x.div(y));
    println!("{} % {} = {}", x, y, x.rem(y));
}

#[allow(dead_code)]
pub fn overflow() {
    let x: u8 = 100;
    let y: u8 = 200;
    let result = x + y;
    println!("{} + {} = {}", x, y, result);
}

#[allow(dead_code)]
pub fn ignore_overflow() {
    let x: u8 = 100;
    let y: u8 = 200;
    let result = x.wrapping_add(y);
    println!("{} + {} = {}", x, y, result);
}

#[allow(dead_code)]
pub fn check_overflow() {
    let x: u8 = 100;
    let y: u8 = 200;
    match x.checked_add(y) {
        Some(z) => println!("{} + {} = {}", x, y, z),
        None => println!("Overflow occurred"),
    }
}

#[allow(dead_code)]
pub fn check_bool_overflow() {
    let x: u8 = 100;
    let y: u8 = 200;
    let (result, overflow) = x.overflowing_add(y);
    if overflow {
        println!("Overflow occurred")
    } else {
        println!("{} + {} = {}", x, y, result);
    }
}

#[allow(dead_code)]
pub fn return_max_overflow() {
    let x: u8 = 100;
    let y: u8 = 200;
    let result = x.saturating_add(y);
    if result == u8::MAX {
        println!("Overflow occurred")
    } else {
        println!("{} + {} = {}", x, y, result);
    }
}
