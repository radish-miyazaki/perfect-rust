#![allow(dead_code)]

pub fn char_literal() {
    println!("a = {}", 'a');
    println!("b = {}", 'b');
    println!("あ = {}", 'あ');
    println!("い = {}", 'い');
}

pub fn char_constant() {
    println!("MAX = {}", char::MAX);
    println!("UNICODE_VERSION = {:?}", char::UNICODE_VERSION);
    println!("REPLACEMENT_CHARACTER = {:?}", char::REPLACEMENT_CHARACTER);
}

pub fn methods() {
    let x = 'a';
    println!("is_alphabetic = {}", x.is_alphabetic());
    println!("is_numeric = {}", x.is_numeric());
    println!("is_lowercase = {}", x.is_lowercase());
    println!("is_uppercase = {}", x.is_uppercase());
    println!("to_ascii_lowercase = {}", x.to_ascii_lowercase());
    println!("to_ascii_uppercase = {}", x.to_ascii_uppercase());
}