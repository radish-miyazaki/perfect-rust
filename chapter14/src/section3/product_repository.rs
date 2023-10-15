use tokio_postgres::Transaction;
use async_trait::async_trait;
use anyhow::Result;

use crate::section1::entity::{Product};
use crate::section3::repository::AsyncRepository;

pub struct ProductRepository<'a, 'b> {
    tx: &'a mut Transaction<'b>,
}

impl<'a, 'b> ProductRepository<'a, 'b> {
    pub fn new(tx: &'a mut Transaction<'b>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl AsyncRepository<Product, i64, bool> for ProductRepository<'_, '_> {
    async fn select_all(&mut self) -> Result<Vec<Product>> {
        use super::sql::get_sql;

        let sql = get_sql("product", "select_all").await?;
        let rows = self.tx.query(sql.as_str(), &[]).await?;
        let mut products = Vec::new();
        for row in rows {
            products.push(Product::new(
                    row.get("id"),
                    row.get("name"),
                    row.get("price"),
                    row.get("category_id"),
                    None
            ))
        }
        Ok(products)
    }

    async fn select_by_id(&mut self, id: i64) -> Result<Option<Product>> {
        let sql = "SELECT id, name, price, category_id FROM product WHERE id = $1";
        let row = self.tx.query_one(sql, &[&id]).await?;
        let product = Product::new(
            row.get("id"),
            row.get("name"),
            row.get("price"),
            row.get("category_id"),
            None
        );
        Ok(Some(product))
    }

    async fn insert(&mut self, _row: Product) -> Result<bool> {
        todo!()
    }


    async fn update_by_id(&mut self, _id: i64) -> Result<bool> {
        todo!()
    }

    async fn delete_by_id(&mut self, _id: i64) -> Result<bool> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::Result;
    use crate::section3::connect::AsyncSimpleClient;
    use crate::section3::transaction::AsyncTransaction;

    #[tokio::test]
    async fn select_all() -> Result<()> {
        let mut client = AsyncSimpleClient::connect().await?;
        let mut tx = AsyncTransaction::start(&mut client, true).await?;

        let mut product_repository = ProductRepository::new(&mut tx);

        let products = product_repository.select_all().await?;
        for product in products {
            println!("{:?}", product);
        }

        Ok(())
    }
}

