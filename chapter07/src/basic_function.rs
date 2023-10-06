#![allow(dead_code)]

pub fn print_message_1() {
    println!("basic definition to function");
}

pub fn print_message_2(msg: String) {
    println!("{}", &msg);
}

pub fn print_message_3(msg: &mut String) {
    msg.push_str("with mutable args");
    println!("result = {}", msg);
}

pub fn print_message_4(msg: String) -> String {
    if msg.is_empty() {
        return String::from("empty");
    }
    println!("{}", msg);
    String::from("args are output")
}
