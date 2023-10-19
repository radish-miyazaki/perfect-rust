use anyhow::{Error, Result};
use async_trait::async_trait;
use env_logger;
use sea_orm::{NotSet, Set, TransactionTrait, ColumnTrait, DatabaseTransaction, IntoActiveModel, QueryOrder, ActiveModelTrait, ModelTrait, Statement, DbBackend, ConnectionTrait};
use sea_orm::EntityTrait;
use sea_orm::query::QueryFilter;

use crate::models::product::{Model, ActiveModel};
use crate::models::prelude::{Product, ProductCategory};
use crate::repository::Repository;
use crate::models::{product, product_category};

pub struct ProductRepository;

impl ProductRepository {
    fn new() -> Self {
        Self
    }

    async fn select_by_join_product_category(&self, tx: &DatabaseTransaction, id: i64) -> Result<Vec<(product::Model, Option<product_category::Model>)>> {
        let result = Product::find_by_id(id)
            .find_also_related(ProductCategory)
            .all(tx)
            .await?;
        Ok(result)
    }

    async fn select_by_id_stmt(&self, tx: &DatabaseTransaction, id: i64) -> Result<Model> {
        let stmt = Statement::from_sql_and_values(
            DbBackend::Postgres,
            "SELECT id, name, price, category_id FROM product WHERE id = $1",
            vec![id.into()]
        );
        let result = Product::find().from_raw_sql(stmt).one(tx).await?;
        match result {
            Some(r) => Ok(r),
            None => Err(Error::msg(format!("id: {} is not found", id)))
        }
    }

    async fn insert_stmt(&self, tx: DatabaseTransaction, row: ActiveModel) -> Result<u64> {
        let stmt = Statement::from_sql_and_values(
            DbBackend::Postgres,
            "INSERT INTO product (name, price, category_id) VALUES ($1, $2, $3)",
            vec![row.name.unwrap().into(), row.price.unwrap().into(), row.category_id.unwrap().into()]
        );
        let result = tx.execute(stmt).await?;
        tx.commit().await?;

        Ok(result.rows_affected())
    }
}

#[async_trait]
impl Repository for ProductRepository {
    type Transaction = DatabaseTransaction;
    type Model = Model;
    type ActiveModel = ActiveModel;

    async fn select_all(&self, tx: &Self::Transaction) -> Result<Vec<Self::Model>> {
        let result = Product::find().all(tx).await?;
        Ok(result)
    }

    async fn select_by_id(&self, tx: &Self::Transaction, id: i64) -> Result<Self::Model> {
        let result = Product::find_by_id(id).one(tx).await?;
        match result {
            Some(r) => Ok(r),
            None => Err(Error::msg(format!("id: {} is not found", id)))
        }
    }

    async fn select_by_name_like(&self, tx: &Self::Transaction, keyword: &str) -> Result<Vec<Self::Model>> {
        let results = Product::find()
            .filter(product::Column::Name.contains(keyword))
            .order_by_asc(product::Column::Id)
            .all(tx).await?;

        Ok(results)
    }

    async fn insert(&self, tx: Self::Transaction, row: Self::ActiveModel) -> Result<Self::Model> {
        let new_product: product::ActiveModel = row.into_active_model();
        let result = new_product.insert(&tx).await?;
        tx.commit().await?;

        Ok(result)
    }

    async fn update_by_id(&self, tx: Self::Transaction, row: Self::ActiveModel) -> Result<Self::Model> {
        let id = row.id.unwrap();
        let target = Product::find_by_id(id).one(&tx).await?;
        if target.is_none() {
            return Err(Error::msg(format!("id: {} is not found", id)));
        }

        let mut update_row = target.unwrap().into_active_model();
        update_row.name = row.name;
        update_row.price = row.price;
        update_row.category_id = row.category_id;

        let result = update_row.update(&tx).await?;
        tx.commit().await?;
        Ok(result)
    }

    async fn delete_by_id(&self, tx: Self::Transaction, id: i64) -> Result<u64> {
        let target = Product::find_by_id(id).one(&tx).await?;
        if target.is_none() {
            return Err(Error::msg(format!("id: {} is not found", id)));
        }

        let result = target.unwrap().delete(&tx).await?;
        tx.commit().await?;
        Ok(result.rows_affected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::pool::SamplePool;

    #[tokio::test]
    async fn select_all() -> Result<()> {
        env_logger::builder().filter_level(log::LevelFilter::Debug).init();

        let pool = SamplePool::get().await?;
        let tx = pool.begin().await?;
        let repository = ProductRepository::new();

        let results = repository.select_all(&tx).await?;
        for result in results {
            println!("{:?}", result);
        }

        Ok(())
    }

    #[tokio::test]
    async fn select_by_id() -> Result<()> {
        env_logger::builder().filter_level(log::LevelFilter::Debug).init();

        let pool = SamplePool::get().await?;
        let tx = pool.begin().await?;
        let repository = ProductRepository::new();

        let result = repository.select_by_id(&tx, 1).await?;
        println!("{:?}", result);

        Ok(())
    }

    #[tokio::test]
    async fn select_by_name_like() -> Result<()> {
        env_logger::builder().filter_level(log::LevelFilter::Debug).init();

        let pool = SamplePool::get().await?;
        let tx = pool.begin().await?;
        let repository = ProductRepository::new();

        let results = repository.select_by_name_like(&tx, "ボールペン").await?;
        for result in results {
            println!("{:?}", result);
        }

        Ok(())
    }

    #[tokio::test]
    async fn insert() -> Result<()> {
        env_logger::builder().filter_level(log::LevelFilter::Debug).init();

        let pool = SamplePool::get().await?;
        let tx = pool.begin().await?;
        let repository = ProductRepository::new();

        let new_product = product::ActiveModel {
            id: NotSet,
            name: Set("ボールペン".to_string()),
            price: Set(200),
            category_id: Set(Some(1)),
        };
        let result = repository.insert(tx, new_product).await?;
        println!("{:?}", result);

        Ok(())
    }

    #[tokio::test]
    async fn update_by_id() -> Result<()> {
        env_logger::builder().filter_level(log::LevelFilter::Debug).init();

        let pool = SamplePool::get().await?;
        let tx = pool.begin().await?;
        let repository = ProductRepository::new();

        let new_product = product::ActiveModel {
            id: Set(1),
            name: Set("ボールペン".to_string()),
            price: Set(200),
            category_id: Set(Some(1)),
        };
        let result = repository.update_by_id(tx, new_product).await?;
        println!("{:?}", result);

        Ok(())
    }

    #[tokio::test]
    async fn delete_by_id() -> Result<()> {
        env_logger::builder().filter_level(log::LevelFilter::Debug).init();

        let pool = SamplePool::get().await?;
        let tx = pool.begin().await?;
        let repository = ProductRepository::new();

        let result = repository.delete_by_id(tx, 1).await?;
        println!("{:?}", result);

        Ok(())
    }

    #[tokio::test]
    async fn select_by_join_product_category() -> Result<()> {
        env_logger::builder().filter_level(log::LevelFilter::Debug).init();

        let pool = SamplePool::get().await?;
        let tx = pool.begin().await?;
        let repository = ProductRepository::new();

        let result = repository.select_by_join_product_category(&tx, 2).await?;
        println!("{:?}", result);

        Ok(())
    }

    #[tokio::test]
    async fn select_by_id_stmt() -> Result<()> {
        env_logger::builder().filter_level(log::LevelFilter::Debug).init();

        let pool = SamplePool::get().await?;
        let tx = pool.begin().await?;
        let repository = ProductRepository::new();

        let result = repository.select_by_id_stmt(&tx, 2).await?;
        println!("{:?}", result);

        Ok(())
    }

    #[tokio::test]
    async fn insert_stmt() -> Result<()> {
        env_logger::builder().filter_level(log::LevelFilter::Debug).init();

        let pool = SamplePool::get().await?;
        let tx = pool.begin().await?;
        let repository = ProductRepository::new();

        let new_product = ActiveModel {
            id: NotSet,
            name: Set("ボールペン".to_string()),
            price: Set(200),
            category_id: Set(Some(1)),
        };
        let result = repository.insert_stmt(tx, new_product).await?;
        println!("{:?}", result);

        Ok(())
    }
}
