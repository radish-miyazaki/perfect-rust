#![allow(dead_code)]

pub fn life_time_1() {
    let x = vec![1, 2, 3];
    let a = String::from("ABC");

    let y = &x;
    let b = &a;

    println!("y = {:?}", y);
    println!("b = {}", b);
    println!("b = {}", b);
    println!("Finished program");
}

pub fn life_time_2() {
    let a = String::from("ABC");

    let b = &a;
    let c = b;

    println!("b = {:?}", b);
    println!("c = {:?}", c);
    println!("Finished program");
}

// pub fn life_time_3() -> &String {
//     let x = String::from("ABC");
//     &x
// }

// fn compare(val1: &String, val2: &String) -> &String {
//     if val1.len() > val2.len() {
//         val1
//     } else {
//         val2
//     }
// }

pub fn life_time_4() {
    let a = String::from("ABC");
    let b = String::from("DEF");

    let result = compare(&a, &b);
    println!("result = {}", result);
}

fn compare<'a>(val1: &'a String, val2: &'a String) -> &'a String {
    if val1.len() > val2.len() {
        val1
    } else {
        val2
    }
}

pub fn life_time_5() {
    let r;
    let a = String::from("ABC");
    {
        let b = String::from("DEF");
        r = compare(&a, &b);
        println!("{:?}", r);
    }
}

pub fn push(val: &mut Vec<i32>) -> &Vec<i32> {
    val.push(10);
    val.push(11);
    val.push(12);
    val
}
