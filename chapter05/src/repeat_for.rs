#![allow(dead_code)]

pub fn for_1() {
    println!("loop from 0 to 4");
    for i in 0..5 {
        println!("i = {}", i);
    }

    for i in (0..5).rev() {
        println!("i = {}", i);
    }
}

pub fn for_2() {
    let values = vec![100, 200, 300, 400, 500];
    let mut result: i32 = 0;
    for value in values {
        result += value;
    }
    println!("result = {}", result);
}
