#![allow(dead_code)]

pub fn value_setting() {
    let mut x: Result<i32, String>;
    x = Ok(100);
    println!("x = {:?}", x);

    x = Err(String::from("Error"));
    println!("x = {:?}", x);
}

fn div(val1: i32, val2: i32) -> Result<i32, String> {
    if val2 == 0 {
        return Err("Zero division error".to_string());
    }

    Ok(val1 / val2)
}

pub fn use_div() {
    let x = 10;
    let y = 5;

    let r = match div(x, y) {
        Ok(res) => format!("{} / {} = {}", x, y, res),
        Err(err) => err,
    };
    println!("{}", r)
}

pub fn method_1() {
    let x = 10;
    let y = 5;

    let result = div(x, y);
    let r = if result.is_ok() {
        format!("{} / {} = {}", x, y, result.unwrap())
    } else {
        result.unwrap_err()
    };
    println!("{}", r);
}

pub fn method_2() {
    let x = 10;
    let y = 0;

    let r = div(x, y).unwrap_or(-100);
    println!("unwrap_or() = {}", r);

    let r = div(x, y).unwrap_or_else(|err| {
        println!("{:?}", err);
        -100
    });
    println!("unwrap_or_else() = {}", r);

    let r = div(x, y).unwrap_or_default();
    println!("unwrap_or_default() = {}", r);
}

pub fn method_3() {
    let x = 10;
    let y = 0;

    let r = div(x, y).and_then(|res| Ok(res * 2));
    println!("and_then() = {:?}", r);

    let r = div(x, y).map(|res| res * 2);
    println!("map() = {:?}", r);

    let r = div(x, y).map_err(|err| err);
    println!("map_err() = {:?}", r);

    let r = div(x, y).map_or(-100, |res| res);
    println!("map_or() = {}", r);

    let r = div(x, y).map_or_else(|err| err, |res| res.to_string());
    println!("map_or_else() = {}", r);

    let r = div(x, y).or_else(|err| Err(err));
    println!("or_else() = {:?}", r);
}

pub fn method_4() {
    let x = 10;
    let y = 5;
    let r = div(x, y).ok();
    println!("ok() = {:?}", r);

    let x = 10;
    let y = 0;
    let r = div(x, y).err();
    println!("err() = {:?}", r);
}

pub fn method_5() -> Result<String, String> {
    let x = 10;
    let y = 5;
    let r = div(x, y)?;
    Ok(format!("{} / {} = {}", x, y, r))
}

