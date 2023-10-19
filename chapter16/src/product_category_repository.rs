use anyhow::Result;
use sea_orm::{DatabaseTransaction, EntityTrait, TransactionTrait};

use crate::models::{product_category, product, prelude::{ProductCategory, Product}};


pub struct ProductCategoryRepository;

impl ProductCategoryRepository {
    fn new() -> Self {
        Self {}
    }

    async fn select_by_id_join_product(&self, tx: &DatabaseTransaction, id: i64)
        -> Result<Vec<(product_category::Model, Vec<product::Model>)>>
    {
        let result = ProductCategory::find_by_id(id)
            .find_with_related(Product)
            .all(tx)
            .await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::pool::SamplePool;

    #[tokio::test]
    async fn test_select_by_id_join_product() -> Result<()> {
        env_logger::builder().filter_level(log::LevelFilter::Debug).init();

        let pool = SamplePool::get().await?;
        let tx = pool.begin().await?;
        let repository = ProductCategoryRepository::new();

        let result = repository.select_by_id_join_product(&tx, 1).await?;
        println!("{:?}", result);
        Ok(())
    }
}


