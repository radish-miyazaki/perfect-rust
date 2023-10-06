#![allow(dead_code)]

use std::collections::LinkedList;

pub fn instantiate() {
    let string_list: LinkedList<String> = LinkedList::new();
    println!("len() = {}", string_list.len());
    println!("is_empty() = {}", string_list.is_empty());
}

pub fn add() {
    let mut list_a: LinkedList<String> = LinkedList::new();
    list_a.push_back("ABC".to_string());
    list_a.push_back("DEF".to_string());
    println!("list_a = {:?}", list_a);

    let mut list_b: LinkedList<String> = LinkedList::new();
    list_b.push_back("OPQ".to_string());
    list_b.push_back("RST".to_string());
    list_a.append(&mut list_b);
    println!("list_a = {:?}", list_a);

    list_a.push_front("XYZ".to_string());
    println!("list_a = {:?}", list_a);
}

pub fn change() {
    let mut list_a = LinkedList::<u32>::new();
    list_a.extend([10, 20, 30, 40, 50, 60, 70, 80, 90]);

    for val in list_a.iter_mut() {
        if *val % 4 == 0 {
            *val *= 10;
        }
    }
    println!("list_a = {:?}", list_a);

    match list_a.back_mut() {
        None => {},
        Some(x) => *x *= 5,
    }
    println!("list_a = {:?}", list_a);
}

pub fn remove() {
    let mut list_a = LinkedList::<u32>::new();
    list_a.extend([10, 20, 30, 40, 50, 60, 70, 80, 90]);

    let r = list_a.pop_front();
    println!("deleted element = {}, result = {:?}", r.unwrap(), list_a);

    let r = list_a.pop_back();
    println!("deleted element = {}, result = {:?}", r.unwrap(), list_a);

    list_a.clear();
    println!("clear() = {:?}", list_a);
}
