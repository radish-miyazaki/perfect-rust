#![allow(dead_code)]

struct Member<'a> {
    id: u32,
    name: &'a str,
    address: &'a str,
    email: &'a str,
}

impl<'a> Member<'a> {
    fn new(id: u32, name: &'a str, address: &'a str, email: &'a str) -> Self {
        Self { id, name, address, email }
    }

    fn get_name(&self) -> &str {
        self.name
    }

    fn set_name(&mut self, name: &'a str) {
        self.name = name
    }

    fn set_and_return_name(&mut self, name: &'a str) -> &str {
        self.name = name;
        self.name
    }
}

pub fn use_method() {
    let mut m = Member::new(100, "山田太郎", "東京都港区", "yamada@example.com");
    m.set_name("山田花子");
    println!("name = {}", m.get_name());

    let r = m.set_and_return_name("山田二郎");
    println!("r = {}", r);
}
