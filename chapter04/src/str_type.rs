#![allow(dead_code)]

pub fn declare() {
    let str = "Hello, Rust!";
    println!("value = {:?}, ptr = {:p}, len = {}", str, str, str.len());
}

pub fn binding() {
    let str_a = "Hello ";
    let str_b = "Rust.";
    println!("str_a:value = {:?}, ptr = {:p}, len = {}", str_a, str_a, str_a.len());
    println!("str_b:value = {:?}, ptr = {:p}, len = {}", str_b, str_b, str_b.len());

    let str_c = str_a.to_string() + str_b;
    println!("str_c:value = {:?}, ptr = {:p}, len = {}", str_c, &str_c, str_c.len());
}

pub fn methods_1() {
    let str_value = "ABCDEF";
    let result = str_value.as_bytes();
    println!("as_bytes() = {:?}", result);

    let result = str_value.bytes();
    println!("bytes() = {:?}", result);

    let result = str_value.chars();
    println!("chars() = {:?}", result);

    let str_value = "123";
    let result = str_value.parse::<i32>();
    println!("parse() = {:?}", result);
}

pub fn methods_2() {
    let str_value = "山田太郎";
    let result = str_value.contains("田");
    println!("contains() = {:?}", result);

    let result = str_value.find("田");
    println!("find() = {:?}", result);

    let str_value = "sample.txt";
    let result = str_value.ends_with(".txt");
    println!("ends_with() = {:?}", result);

    let result = str_value.starts_with("sample");
    println!("starts_with() = {:?}", result);

    let result = str_value.is_empty();
    println!("is_empty() = {}", result);
}

pub fn methods_3() {
    let str_value = "ABC\r\nDEF\r\nXYZ\r\n";
    let result = str_value.lines();
    for row in result {
        println!("lines() = {:?}", row);
    }

    let str_value = "ABCDEFXYZ";
    let result = str_value.get(0..=3).unwrap();
    println!("get() = {:?}", result);

    let result = str_value.get(..).unwrap();
    println!("get() = {:?}", result);
}

pub fn methods_4() {
    let str_value = "ABCDEFXYZ";
    let result = str_value.repeat(3);
    println!("repeat() = {:?}", result);

    let result = str_value.replace("ABC", "abc");
    println!("replace() = {:?}", result);
}

pub fn methods_5() {
    let str_value = "ABCDEFXYZ";
    let result = str_value.to_lowercase();
    println!("to_lowercase() = {:?}", result);

    let str_value = "abcdefxyz";
    let result = str_value.to_uppercase();
    println!("to_uppercase() = {:?}", result);

    let str_value = " Hello Rust ";
    let result = str_value.trim();
    println!("trim() = {:?}", result);

    let str_value = "ABC,DEF,XYZ";
    let result = str_value.split(",");
    for val in result {
        println!("split() = {:?}", val);
    }
}

