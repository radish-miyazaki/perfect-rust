#![allow(dead_code)]

pub fn ownership_1() {
    let x = String::from("ABC");
    println!("x = {:?}", x);

    let y = x;
    // println!("x = {:?}", x);
    println!("y = {:?}", y);
}

pub fn ownership_2() {
    let x = String::from("ABC");
    println!("x = {:?}", x);

    let y = &x;
    println!("x = {:?}", x);
    println!("y = {:?}", y);
}

pub fn ownership_3() {
    let x = String::from("ABC");
    println!("x = {:?}", x);

    let y = x.clone();
    println!("x = {:p}", &x);
    println!("y = {:p}", &y);
}

fn print_message(msg: &String) {
    println!("msg = {:?}", msg);
}

pub fn ownership_5() {
    let x = String::from("ABC");
    print_message(&x);
    println!("x = {:?}", x);
}

fn message() -> String {
    let r = String::from("Good morning");
    r
}

pub fn ownership_6() {
    let x = message();
    println!("x = {:?}", x);
}

pub fn ownership_7() {
    {
        let x = String::from("ABC");
        println!("x = {:?}", x);
    }
    // let y = &x;
    // println!("y = {:?}", y);
}
