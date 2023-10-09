#![allow(dead_code)]

struct Customer {
    id: u32,
    name: String,
    address: String,
    email: String,
}

impl Customer {
    fn new(id: u32, name: String, address: String, email: String) -> Self {
        Self { id, name, address, email }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

pub fn use_method() {
    let mut c = Customer::new(100, String::from("山田太郎"), String::from("東京都港区"), String::from("yamada@example.com"));
    c.set_name(String::from("山田花子"));
    println!("name = {}", c.get_name());
}

