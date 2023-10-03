#![allow(dead_code)]

pub fn branch_1() {
    let x = 10;
    match x {
        1 => println!("value is 1"),
        2 => println!("value is 2"),
        _ => println!("value is invalid"),
    }

    match x {
        1 => {
            let y = 100;
            println!("y is {}", y);
        },
        2 => {
            let y = 200;
            println!("y is {}", y);
        },
        _ => {
            let y = 300;
            println!("y is {}", y);
        }
    }
}

pub fn branch_2() {
    let x = "山田太郎";
    match x {
        "山田太郎" => println!("I'm 山田太郎"),
        "鈴木花子" => println!("I'm 鈴木花子"),
        _ => println!("Who am I?"),
    }
}

pub fn branch_3() {
    let calc = |x: i32| { x * 10 };
    let y = 3;
    let result = match y {
        1 => calc(10),
        2 => calc(20),
        3 => calc(30),
        _ => calc(0),
    };
    println!("result = {}", result);
}

pub fn branch_4() {
    let calc = |x: u32| { x * 10 };
    let value = 30;
    let result = match value {
        1..=3 => calc(10),
        4..=6 => calc(20),
        7..=9 => calc(30),
        10 | 20 | 30 => calc(40),
        21..=25 | 31 => calc(50),
        _ => calc(0),
    };
    println!("result = {}", result);
}

pub fn branch_5() {
    let value = (10, 25);
    let result = match value {
        (x, y) if x == 0 && y == 0 => "x and y are both 0",
        (x, y) if x % 2 == 0 && y % 2 == 0 => "x and y are both even",
        (x, y) if x % 2 == 1 && y % 2 == 1 => "x and y are both odd",
        _ => "does not match any pattern",
    };
    println!("result = {}", result);
}
