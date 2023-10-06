#![allow(dead_code)]

pub fn instantiate() {
    let vec: Vec<i32> = Vec::with_capacity(5);
    println!("pointer = {:p}, length = {}, capacity = {}", &vec, vec.len(), vec.capacity());

    let vec = vec![(10, 20), (100, 200)];
    println!("pointer = {:p}, length = {}, capacity = {}", &vec, vec.len(), vec.capacity());
}

pub fn add() {
    let mut vec: Vec<i32> = Vec::with_capacity(5);
    for value in 0..10 {
        vec.push(value);
    }
    println!("value = {:?}, length = {}, capacity = {}", vec, vec.len(), vec.capacity());

    let mut vec = vec![(10, 20), (100, 200)];
    vec.insert(1, (1000, 2000));
    println!("value = {:?}, length = {}, capacity = {}", vec, vec.len(), vec.capacity());
}

pub fn get_and_change() {
    let mut vec = vec![1, 2, 3, 4, 5];
    println!("second value = {}", vec[1]);

    vec[2] *= 100;
    println!("{:?}", vec);
}

pub fn remove() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let x = nums.drain(2..5);
    println!("deleted elements = {:?}", x.collect::<Vec<i32>>());
    println!("result = {:?}", nums);

    let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut index = 0;
    while index < nums.len() {
        if nums[index] % 2 == 0 {
            let num = nums.remove(index);
            println!("deleted element = {:?}", num);
        } else {
            index += 1;
        }
    }
    println!("even numbers = {:?}", nums);
}

pub fn sort() {
    let mut nums = vec![8, 5, 3, 1, 4, 6, 2, 5, 9, 2];
    nums.sort();
    println!("sort() = {:?}", nums);

    nums.dedup();
    println!("dedup() = {:?}", nums);

    use std::cmp::Reverse;
    let mut nums = vec![8, 5, 3, 1, 4, 6, 2, 5, 9, 2];
    nums.sort_by_key(|el| Reverse(*el));
    println!("sort_by_key() = {:?}", nums);
}

