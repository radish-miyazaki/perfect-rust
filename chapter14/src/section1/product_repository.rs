use postgres::Transaction;

use super::{entity::Product, repository::Repository};

pub struct ProductRepository<'a, 'b> (pub &'a mut Transaction<'b>);
impl Repository<Product, i32, u64> for ProductRepository<'_, '_> {}
