#![allow(dead_code)]

pub fn branch_1() {
    let name = "山田太郎";
    if name.eq("山田太郎") {
        println!("name = 山田太郎");
    }

    let num = 120;
    if num % 2 == 0 {
        println!("num is even");
    } else {
        println!("num is odd");
    }
}

pub fn branch_2() {
    let num = 10;
    if num == 1 {
        println!("num is 1");
    } else if num == 2 {
        println!("num is 2");
    } else {
        println!("num is neither 1 nor 2");
    }
}

pub fn branch_3() {
    let num = 10;
    let result = if num == 1 {
        "variable num value is 1"
    } else if num == 2 {
        "variable num value is 2"
    } else {
        "variable num value is neither 1 nor 2"
    };
    println!("result = {}", result)
}
