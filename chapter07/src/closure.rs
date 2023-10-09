#![allow(dead_code)]

pub fn closure_sum() {
    let sum = |vals: &Vec<i32>| {
        let mut sum = 0;
        for val in vals {
            sum += val;
        }
        sum
    };

    let vals = vec![1, 2, 3, 4, 5];
    println!("sum = {}", sum(&vals));
}

pub fn move_1() {
    let values = vec![1, 2, 3, 4, 5];
    let sum = || {
        let mut sum = 0;
        for value in values.iter() {
            sum += value;
        }
        sum
    };
    println!("sum = {}", sum());
    println!("values = {:?}", values);
}

pub fn move_2() {
    let values = vec![1, 2, 3, 4, 5];
    let sum = move || {
        let mut sum = 0;
        for value in values {
            sum += value;
        }
        sum
    };
    println!("sum = {}", sum());
}

pub fn impl_1(values: Vec<i32>) -> impl FnOnce() -> i32 {
    move || {
        let mut sum = 0;
        for value in values {
            sum += value;
        }
        sum
    }
}

pub fn where_1<F>(f: F) where F: FnOnce() -> i32 {
    let sum = f();
    println!("sum = {}", sum);
}

pub fn impl_2() -> impl Fn(Vec<i32>) -> i32 {
    |values: Vec<i32>| {
        let mut sum = 0;
        for value in values {
            sum += value;
        }
        sum
    }
}

pub fn where_2<F>(f: F) where F: Fn(Vec<i32>) -> i32 {
    let values = vec![1, 2, 3, 4, 5];
    let sum = f(values);
    println!("sum = {}", sum);
}
