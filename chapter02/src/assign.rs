#[allow(dead_code)]
pub fn assign_value(x: i64, y: i64) {
    let a = x;
    let b = y;
    println!("変数aの値 = {}", a);
    println!("変数bの値 = {}", b);
}

#[allow(dead_code)]
pub fn compound_assign(mut x: i32, y: i32) {
    x += y;
    println!("x += y = {}", x);
    x -= y;
    println!("x -= y = {}", x);
    x *= y;
    println!("x *= y = {}", x);
    x /= y;
    println!("x /= y = {}", x);
    x %= y;
    println!("x %= y = {}", x);
}

#[allow(dead_code)]
pub fn compound_assign_method(mut x: i32, y: i32) {
    use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
    x.add_assign(y);
    println!("x += y = {}", x);
    x.sub_assign(y);
    println!("x -= y = {}", x);
    x.mul_assign(y);
    println!("x *= y = {}", x);
    x.div_assign(y);
    println!("x /= y = {}", x);
    x.rem_assign(y);
    println!("x %= y = {}", x);
}
