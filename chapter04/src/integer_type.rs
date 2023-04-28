#![allow(dead_code)]
pub fn integer_literal() {
    println!("a = {}", 100i64);
    println!("b = {}", 0o123u64);
    println!("c = {}", 0xf124i32);
    println!("d = {}", 0o123_u64);
    println!("e = {}", 0x_f123i32);
    println!("f = {}", 123_456_789);
}

#[allow(dead_code)]
pub fn byte_literal() {
    println!("a = {}", b'\x01');
    println!("b = {}", b'\x0A');
    println!("c = {}", b'\x1D');
}

#[allow(dead_code)]
pub fn i32_contant() {
    println!("BITS = {}", i32::BITS);
    println!("MAX = {}", i32::MAX);
    println!("MIN = {}", i32::MIN);
}

#[allow(dead_code)]
pub fn methods() {
    println!("abs() = {}", (-10i32).abs());
    println!("signum() = {}", -10i32.signum());
    println!("pow() = {}", 10i32.pow(3));
    println!("is_negative() = {}", 10i32.is_negative());
    println!("is_positive() = {}", 10i32.is_positive());

    let x = 1000;
    let y = x.clone();
    println!("clone() = {}", y);
    println!("to_string() = {}", y.to_string());
}
