#![allow(dead_code)]

type Calc = fn(i32, i32) -> i32;

fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub fn use_function_1() {
    let func = add;
    let result = func(10, 20);
    println!("x + y = {}", result);
}

fn use_calc_type(func: Calc, x: i32, y: i32) -> i32 {
    func(x, y)
}

pub fn use_function_2() {
    let calc: Calc = add;
    let result = use_calc_type(calc, 10, 20);
    println!("x + y = {}", result);
}

fn return_calc_type() -> Calc {
    add
}

pub fn use_function_3() {
    let calc = return_calc_type();
    let r = calc(10, 20);
    println!("10 + 20 = {}", r);
}
