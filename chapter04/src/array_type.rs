#![allow(dead_code)]

pub fn declare() {
    let array_a = [1, 2, 3];
    let array_b: [i32; 3] = [1, 2, 3];
    let array_c = [0; 3];

    println!("array_a = {:?}", array_a);
    println!("array_b = {:?}", array_b);
    println!("array_c = {:?}", array_c);
}

pub fn use_array() {
    let mut array_a = [0; 3];
    array_a[0] = 100;
    array_a[1] = 200;
    array_a[2] = 300;

    for value in array_a {
        println!("value = {}", value);
    }
}

pub fn methods() {
    let mut array_a: [i32; 5] = [2, 3, 5, 4, 12];

    println!("is_empty() = {}", array_a.is_empty());
    println!("contains() = {}", array_a.contains(&3));
    println!("first() = {:?}", array_a.first());
    println!("last() = {:?}", array_a.last());

    array_a.sort();
    println!("sort() = {:?}", array_a);

    array_a.reverse();
    println!("reverse() = {:?}", array_a);
}

pub fn multi_dimensional() {
    let array_a = [[0; 5]; 3];
    for sub_array in array_a {
        println!("配列 sub_array = {:?}", sub_array);
    }

    let array_b = [[[10; 5]; 3]; 2];
    for sub_array in array_b {
        println!("二次元配列 sub_array = {:?}", sub_array);
    }
}
