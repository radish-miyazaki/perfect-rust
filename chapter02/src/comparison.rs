#[allow(dead_code)]
pub fn symbol(x: i32, y: i32) {
    println!("{} == {} = {}", x, y, x == y);
    println!("{} != {} = {}", x, y, x != y);
    println!("{} > {} = {}", x, y, x > y);
    println!("{} < {} = {}", x, y, x < y);
    println!("{} >= {} = {}", x, y, x >= y);
    println!("{} <= {} = {}", x, y, x <= y);
}

#[allow(dead_code)]
pub fn methods(x: i32, y: i32) {
    println!("{} eq {} = {}", x, y, x.eq(&y));
    println!("{} ne {} = {}", x, y, x.ne(&y));
    println!("{} gt {} = {}", x, y, x.gt(&y));
    println!("{} lt {} = {}", x, y, x.lt(&y));
    println!("{} ge {} = {}", x, y, x.ge(&y));
    println!("{} le {} = {}", x, y, x.le(&y));
}
