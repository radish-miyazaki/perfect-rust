#[allow(dead_code)]
#[allow(unused_variables)]
pub fn declar_variables() {
    let x: i32 = 0;
    let y = 100;
}

#[allow(dead_code)]
pub fn declar_mutable_variables() {
    let mut x = 100;
    x += 100;
    println!("x = {}", x);
}

#[allow(dead_code)]
pub fn shadowing() {
    let value1: i32 = 100;
    println!("value1 = {}", value1);
    let value1: i32 = 10;
    println!("value1 = {}", value1);

    let value2: f32 = 100.1;
    println!("value2 = {}", value2);
    let value2: i32 = 100;
    println!("value2 = {}", value2);
}
