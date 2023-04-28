#![allow(dead_code)]
pub fn floating_literal() {
    println!("a = {}", 10.5f32);
    println!("b = {}", -10.5f64);
    println!("c = {}", 10.5_f32);
    println!("d = {}", -10.5_f64);
}
