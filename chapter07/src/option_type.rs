#![allow(dead_code)]

pub fn declare() {
    let mut a = None;
    println!("a = {:?}", a);

    a = Some(100);
    println!("a = {:?}", a);

    let mut b = Some(String::from("ABC"));
    println!("b = {:?}", b);

    b = None;
    println!("b = {:?}", b);
}

fn div(val1: i32, val2: i32) -> Option<i32> {
    if val2 == 0 {
        return None;
    }

    let r = val1 / val2;
    Some(r)
}

pub fn use_div() {
    let x = 10;
    let y = 5;
    let r = match div(x, y) {
        None => "Doesn't divide".to_string(),
        Some(res) => format!("{} / {} = {}", x, y, res),
    };
    println!("{}", r);
}

pub fn method_1() {
    let x = 10;
    let y = 0;
    let res = div(x, y);

    let r = if res.is_some() {
        format!("{} / {} = {}", x, y, res.unwrap())
    } else {
        "Doesn't divide".to_string()
    };
    println!("{}", r);
}

pub fn method_2() {
    let x = 10;
    let y = 0;

    let r = div(x, y).unwrap_or(-1);
    println!("unwrap_or() = {}", r);

    let r = div(x, y).unwrap_or_else(|| -100);
    println!("unwrap_or_else() = {}", r);

    let r = div(x, y).unwrap_or_default();
    println!("unwrap_or_default() = {}", r);
}

pub fn method_3() {
    let x = 10;
    let y = 0;

    let r = div(x, y).and_then(|res| Some(format!("{} / {} = {}", x, y, res)));
    println!("and_then() = {:?}", r);

    let r = div(x, y).map(|res| format!("{} / {} = {}", x, y, res));
    println!("map() = {:?}", r);

    let r = div(x, y).map_or("Doesn't divide".to_string().to_string(),
            |res| format!("{} / {} = {}", x, y, res));
    println!("map_or() = {}", r);

    let r = div(x, y).map_or_else(|| "Doesn't divide".to_string(),
            |res| format!("{} / {} = {}", x, y, res));
    println!("map_or_else() = {}", r);
}

pub fn method_4() {
    let x = 10;
    let y = 5;
    let r = match div(x, y).ok_or("Doesn't divide".to_string()) {
        Ok(res) => format!("{} / {} = {}", x, y, res),
        Err(err) => err,
    };
    println!("ok_or() = {}", r);

    let y = 0;
    let r = match div(x, y).ok_or_else(|| "Doesn't divide".to_string()) {
        Ok(res) => format!("{} / {} = {}", x, y, res),
        Err(err) => err,
    };
    println!("ok_or_else() = {}", r);
}

pub fn method_5() -> Option<String> {
    let x = 10;
    let y = 5;
    let result = div(x, y)?;
    Some(format!("{} / {} = {}", x, y, result))
}
