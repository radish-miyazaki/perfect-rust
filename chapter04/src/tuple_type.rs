#![allow(dead_code)]

pub fn declare() {
    let x: (i32, &str, bool) = (100, "Hello", true);

    let (a, b, c) = (100, "Hello", true);

    let (l, _, m) = x;
    println!("x = {:?}", x);
    println!("a = {:?}, b = {:?}. c = {:?}", a, b, c);
    println!("l = {:?}, m = {:?}", l, m);
}

pub fn calc(value: (i32, i32)) {
    println!("{} + {} = {}", value.0, value.1, value.0 + value.1);
    println!("{} - {} = {}", value.0, value.1, value.0 - value.1);
    println!("{} * {} = {}", value.0, value.1, value.0 * value.1);
    println!("{} / {} = {}", value.0, value.1, value.0 / value.1);
    println!("{} % {} = {}", value.0, value.1, value.0 % value.1);
}

pub fn methods() {
    let a: (i32, i32, i32) = (100, 200, 300);
    println!("clone() = {:?}", a.clone());
    println!("eq() = {}", a.eq(&(100, 200, 300)));
    println!("cmp() = {:?}", a.cmp(&(100, 200, 300)));
    println!("max() = {:?}", a.max((100, 200, 400)));
    println!("min() = {:?}", a.min((10, 20, 30)))
}
