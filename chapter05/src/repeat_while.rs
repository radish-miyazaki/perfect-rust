#![allow(dead_code)]

pub fn while_1() {
    let mut counter = 0;
    while counter < 5 {
        if counter % 2 == 0 {
            println!("counter value {} is even", counter);
        } else {
            println!("counter value {} is odd", counter);
        }
        counter += 1;
    }
}

pub fn while_2() {
    let x = ["ABC", "DEF", "GHI", "JKL", "MNO"];
    let mut counter = 0;
    while let "ABC" = x[counter] {
        println!("x[counter] = {}", x[counter]);
        counter += 1;
    }
}
