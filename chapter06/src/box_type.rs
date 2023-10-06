#![allow(dead_code)]

pub fn instantiate() {
    use std::ops::Add;

    let x = Box::new(100);
    let y = Box::new(200);
    println!("x + y = {}", x.add(*y));
    println!("x + y = {}", *x + *y);

    let a = Box::new("ABCXYZ");
    let b = Box::new("XYZ");
    println!("contains = {}", a.contains(*b));
}
