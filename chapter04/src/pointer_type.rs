#![allow(dead_code)]

pub fn declare() {
    let x: i32 = 100;
    let y: &str = "ABC";

    let x_ptr: *const i32 = &x;
    let y_ptr: *const &str = &y;

    unsafe {
        println!("x_ptr value is {}", *x_ptr);
        println!("y_ptr value is {}", *y_ptr);
    }

    println!("x pointer address is {:?}", x_ptr);
    println!("y pointer address is {:?}", y_ptr);
}

pub fn mut_declare() {
    let mut x: i32 = 100;
    let mut y: &str = "ABC";

    let x_ptr: *mut i32 = &mut x;
    let y_ptr: *mut &str = &mut y;

    unsafe {
        println!("Before x_ptr value is {}", *x_ptr);
        println!("Before y_ptr value is {}", *y_ptr);

        *x_ptr *= 100;
        let str_value = "Use pointer".to_string();
        *y_ptr = &str_value;

        println!("After x_ptr value is {}", *x_ptr);
        println!("After y_ptr value is {}", *y_ptr);
    }
}

pub fn mut_declare_2() {
    let x: i32 = 100;
    let y: i32 = 200;

    let mut ptr: *const i32 = &x;
    unsafe {
        println!("ptr value is {}", *ptr);
        println!("ptr address is {:?}", ptr);
    }

    ptr = &y;
    unsafe {
        println!("ptr value is {}", *ptr);
        println!("ptr address is {:?}", ptr);
    }
}
