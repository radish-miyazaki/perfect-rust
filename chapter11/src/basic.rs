#![allow(dead_code)]

use std::env::current_dir;
use std::fs::File;
use std::io::ErrorKind;

#[derive(Debug, PartialEq, Eq)]
pub enum ValueConversion {
    Int,
    Float
}

fn parse_01(val: String, conv: ValueConversion) {
    if conv == ValueConversion::Int {
        println!("{:?}", val.parse::<i32>());
    } else {
        println!("{:?}", val.parse::<f32>());
    }
}

pub fn use_parse_01() {
    parse_01(String::from("123"), ValueConversion::Int);
    parse_01(String::from("123"), ValueConversion::Float);

    parse_01(String::from("ABC"), ValueConversion::Int);
    parse_01(String::from("ABC"), ValueConversion::Float);
}

pub fn use_error_kind() {
    let file_path = current_dir()
        .map(|p| p.join("files/sample.txt"))
        .map_err(|e| panic!("{}", e))
        .unwrap();
    let error = File::open(file_path).err().unwrap();

    let result = match error.kind() {
        ErrorKind::NotFound => "Specified file not found",
        ErrorKind::PermissionDenied
            => "Does not have permission to access the file",
        _ => "Unknown error "
    };
    println!("{}", result);
}
