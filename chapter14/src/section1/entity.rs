use lombok::*;

#[derive(Getter, GetterMut, Setter, NoArgsConstructor, AllArgsConstructor, ToString, Clone)]
pub struct ProductCategory {
    id: i64,
    name: String,
    products: Option<Vec<Product>>
}

#[derive(Getter, GetterMut, Setter, NoArgsConstructor, AllArgsConstructor, ToString, Clone)]
pub struct Product {
    id: i64,
    name: String,
    price: i32,
    category_id: i64,
    category: Option<ProductCategory>
}
