use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Customer {
    name: String,
    email: String
}

impl Customer {
    pub fn new(name: String, email: String) -> Self {
        Self { name, email }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}
