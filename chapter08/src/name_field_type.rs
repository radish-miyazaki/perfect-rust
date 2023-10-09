#![allow(dead_code)]

struct Customer {
    id: u32,
    name: String,
    address: String,
    email: String,
}

pub fn generate_1() {
    let customer = Customer {
        id: 100,
        name: String::from("山田太郎"),
        address: String::from("東京都港区"),
        email: String::from("yamada@example.com"),
    };

    println!("id = {}", customer.id);
    println!("name = {}", customer.name);
    println!("address = {}", customer.address);
    println!("email = {}", customer.email);
}

pub struct Member<'a> {
    id: u32,
    name: &'a str,
    address: &'a str,
    email: &'a str,
}
