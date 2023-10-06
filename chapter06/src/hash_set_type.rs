#![allow(dead_code)]

use std::collections::HashSet;

pub fn instantiate() {
    let set_a = HashSet::<i32>::new();
    println!("len() = {}, set_a = {:?}", set_a.len(), set_a);

    let set_b: HashSet::<&str> = HashSet::with_capacity(5);
    println!("len() = {}, set_b = {:?}", set_b.len(), set_b);

    let set_c: HashSet<i32> = vec![10, 20, 30].into_iter().collect();
    println!("len() = {}, set_c = {:?}", set_c.len(), set_c);
}

pub fn add_and_remove() {
    let mut set_a: HashSet<_> = vec![10, 20, 30].into_iter().collect();
    set_a.extend([50, 60, 70]);
    println!("extend() = {:?}", set_a);

    let x = set_a.insert(100);
    if x {
        println!("insert() = {:?}", set_a);
    } else {
        println!("Does not insert.");
    }

    let x = set_a.remove(&100);
    if x {
        println!("remove() = {:?}", set_a);
    } else {
        println!("Does not delete.");
    }

    set_a.retain(|&v| v % 4 == 0);
    println!("retain() = {:?}", set_a);
}

pub fn get() {
    let set_a: HashSet<_> = vec![10, 20, 30].into_iter().collect();
    let r: String = match set_a.get(&10) {
        None => "Not found value.".to_string(),
        Some(x) => format!("Get value = {}", x),
    };
    println!("{}", r);

    let iter_a = set_a.iter();
    for v in iter_a {
        println!("{}", v);
    }
}

pub fn set_op() {
    let set_a: HashSet<_> = vec![10, 20, 30, 50, 60].into_iter().collect();
    let set_b: HashSet<_> = vec![30, 40, 50, 70, 80].into_iter().collect();
    let x = set_a.union(&set_b);
    println!("union() = {:?}", x);

    let x = set_a.intersection(&set_b);
    println!("intersection() = {:?}", x);

    let x = set_a.difference(&set_b);
    println!("difference() = {:?}", x);

    let x = set_a.symmetric_difference(&set_b);
    println!("symmetric_difference() = {:?}", x);
}
