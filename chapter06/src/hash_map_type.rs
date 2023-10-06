#![allow(dead_code)]

use std::collections::HashMap;

pub fn instantiate() {
    let map_x: HashMap<i32, String> = HashMap::new();
    let map_y: HashMap<i32, i32> = HashMap::new();
    println!("map_x = {:?}, map_x.len() = {}", map_x, map_x.len());
    println!("map_y = {:?}, map_y.len() = {}", map_y, map_y.len());
}

pub fn add() {
    let mut map_x: HashMap<i32, &str> = HashMap::new();
    map_x.extend([(1, "ABC"), (2, "DEF"), (3, "GHI")]);
    println!("extend() = {:?}", map_x);

    let mut map_x: HashMap<i32, &str> = HashMap::new();
    map_x.insert(1, "ABC");
    map_x.insert(2, "DEF");
    map_x.insert(3, "GHI");
    println!("insert() = {:?}", map_x);

    let map_x: HashMap<i32, &str> = vec![(1, "ABC"), (2, "DEF"), (3, "GHI")].into_iter().collect();
    println!("vec! = {:?}", map_x);
}

pub fn get_and_change() {
    let mut map_x: HashMap<i32, &str> = HashMap::new();
    map_x.extend([(1, "ABC"), (2, "DEF"), (10, "XYZ")]);

    let v = map_x.get(&100);
    println!("get() = {:?}", v);

    let x_iter = map_x.iter();
    for (k, v) in x_iter {
        println!("key = {}, value = {}", k, v);
    }

    let mut map_x: HashMap<i32, &str> = HashMap::new();
    map_x.extend([(1, "ABC"), (2, "DEF"), (10, "XYZ")]);

    if let Some(v) = map_x.get_mut(&2) {
        *v = "あいうえお";
    }
    println!("{:?}", map_x);

    let mut map_y: HashMap<i32, i32> = vec![(1, 10), (2, 11), (3, 12), (4, 13)].into_iter().collect();
    let y_iter = map_y.iter_mut();
    for (_, v) in y_iter {
        if *v % 2 == 0 {
            *v *= 10;
        }
    }
    println!("{:?}", map_y);
}

pub fn remove() {
    let mut map_x = HashMap::<i32, &str>::new();
    map_x.extend([(1, "ABC"), (2, "DEF"), (10, "XYZ")]);
    let v: String = match map_x.remove(&2) {
        None => "Not found specified key and value.".to_string(),
        Some(x) => format!("value {} is deleted.", x),
    };
    println!("{}", v);
    println!("{:?}", map_x);

    map_x.clear();
    println!("clear() = {:?}", map_x);
}
