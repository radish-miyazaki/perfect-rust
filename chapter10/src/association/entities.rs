use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Product {
    pub id:    String,
    pub name:  String,
    pub price: u32,
}