#![allow(dead_code)]
pub const SAMPLE_NAME: &str = "Rust Sample Code";

#[allow(dead_code)]
pub fn use_constant() {
    const CALC_VALUE: i32 = 100;
    let result = 10 * CALC_VALUE;
    println!("10 * CALC_VALUE = {}", result);
    println!("SAMPLE_NAME = {}", SAMPLE_NAME);
}
