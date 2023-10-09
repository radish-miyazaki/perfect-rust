#![allow(dead_code)]

#[derive(Debug, Clone)]
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

impl Drop for Customer {
    fn drop(&mut self) {
        println!("{} instance is dropped.", self.name);
    }
}

impl Default for Customer {
    fn default() -> Self {
        Self { id: 0, name: String::from(""), address: String::from(""), email: String::from("") }
    }
}

// impl From<&Vec<&str>> for Customer {
//     fn from(v: &Vec<&str>) -> Self {
//         Self { id: v[0].parse::<u32>().unwrap(), name: v[1].to_string(), address: v[2].to_string(), email: v[3].to_string() }
//     }
// }

impl TryFrom<&Vec<&str>> for Customer {
    type Error = String;
    fn try_from(v: &Vec<&str>) -> Result<Self, Self::Error> {
        let id = match v[0].parse::<u32>() {
            Ok(id) => id,
            Err(err) => return Err(err.to_string())
        };
        Ok(Self { id, name: v[1].to_string(), address: v[2].to_string(), email: v[3].to_string() })
    }
}

pub fn use_method() {
    let mut c = Customer::new(100, String::from("山田太郎"), String::from("東京都港区"), String::from("yamada@example.com"));
    c.set_name(String::from("山田花子"));
    println!("name = {}", c.get_name());
}

pub fn use_debug() {
    let c = Customer::new(100, String::from("山田太郎"), String::from("東京都港区"), String::from("yamada@example.com"));
    println!("{:?}", c);
}

pub fn use_clone() {
    let customer = Customer::new(100, String::from("山田太郎"), String::from("東京都港区"), String::from("yamada@example.com"));
    println!("customer.clone() = {:?}", customer.clone());
}

pub fn use_drop() {
    let customer = Customer::new(100, String::from("山田太郎"), String::from("東京都港区"), String::from("yamada@example.com"));
    let mut customer2 = customer.clone();
    customer2.set_name(String::from("田中花子"));
}

pub fn use_default() {
    let customer = Customer::default();
    println!("customer = {:?}", customer);
}

// pub fn use_from() {
//     let value = vec!["100", "山田太郎", "東京都港区", "yamada@example.com"];
//     let customer = Customer::from(&value);
//     println!("customer = {:?}", customer);
// }

pub fn use_try_from() {
    let value = vec!["100", "山田太郎", "東京都港区", "yamada@example.com"];
    let customer = Customer::try_from(&value);
    if customer.is_ok() {
        println!("try_from = {:?}", customer.unwrap());
    } else {
        println!("try_from = {}", customer.unwrap_err());
    }
    // customer.map_or_else(|err| println!("try_from = {}", err), |c| println!("try_from = {:?}", c));
}

