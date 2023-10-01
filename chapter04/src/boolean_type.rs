#[allow(dead_code)]
pub fn is_adult(age: i32) -> bool {
    println!("age = {}", age);

    if age > 19 {
        true
    } else {
        false
    }
}

pub fn method(age: i32) -> String {
    let result: Option<String> = true.then(|| {
        if age > 19 {
            format!("{} is adult", age)
        } else {
            format!("{} is not adult", age)
        }
    });
    result.unwrap()
}
