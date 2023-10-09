#![allow(dead_code)]

#[derive(Debug)]
struct Customer<T> {
    id: T,
    name: String,
    address: String,
    email: String
}

impl<T> Customer<T> {
    fn new(id: T, name: String, address: String, email: String) -> Self {
        Self { id, name, address, email }
    }

    fn change_id(&mut self, val: T) {
        self.id = val;
    }
}

pub fn use_new() {
    let c = Customer::<u64>::new(100, String::from("山田太郎"), String::from("東京都港区"), String::from("yamada@example.com"));
    println!("{:?}", c);
}

pub fn use_change_id() {
    let mut c = Customer::<u64>::new(100, String::from("山田太郎"), String::from("東京都港区"), String::from("yamada@example.com"));
    c.change_id(200);
    println!("{:?}", c);
}
