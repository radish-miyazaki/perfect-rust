use futures_util::StreamExt;
use anyhow::{Error, Result};
use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::Collection;
use mongodb::options::{FindOptions, UpdateModifications};

use crate::client::SampleMongoClient;
use crate::entity::Product;
use crate::repository::Repository;

pub struct ProductRepository {
    collection: Collection<Product>
}

impl ProductRepository {
    pub fn new(client: SampleMongoClient, collection_name: &str) -> Self {
        Self {
            collection: client.get_database().collection(collection_name)
        }
    }
}

#[async_trait]
impl Repository<Product, i32, bool> for ProductRepository {
    async fn select_all(&self) -> Result<Vec<Product>> {
        let opts = FindOptions::builder().sort(doc! { "price": 1}).build();
        let mut cursor = self.collection.find(None, opts).await?;

        let mut products = Vec::new();
        while let Some(product) = cursor.next().await {
            products.push(product?);
        }
        Ok(products)
    }

    async fn select_by_id(&self, id: i32) -> Result<Product> {
        let filter = doc! {"product_id": id};
        self.collection.find_one(filter, None).await?
            .ok_or(Error::msg(format!("product id: {} is not found", id)))
    }

    async fn insert(&self, product: Product) -> Result<bool> {
        self.collection.insert_one(product, None).await.map(|_| Ok(true))?
    }

    async fn insert_many(&self, products: Vec<Product>) -> Result<bool> {
        self.collection.insert_many(products.clone(), None).await.map(|ret| {
            if ret.inserted_ids.iter().count() == products.iter().count() {
                Ok(true)
            } else {
                Ok(false)
            }
        })?
    }

    async fn update_by_id(&self, row: Product) -> Result<bool> {
        let query = doc! {"product_id": row.get_product_id()};
        let update = UpdateModifications::Document(
            doc! {"$set": {
                "name": row.get_name(),
                "price": row.get_price(),
            }}
        );

        let result = self.collection.update_one(query, update, None).await.map(|ret| {
            if ret.modified_count == 1 {
                true
            } else {
                false
            }
        })?;

        Ok(result)
    }

    async fn delete_by_id(&self, id: i32) -> Result<bool> {
        let filter = doc! {"product_id": id};
        let result = self.collection.delete_one(filter, None).await.map(|ret| {
            if ret.deleted_count == 1 {
                true
            } else {
                false
            }
        })?;

        Ok(result)
    }

    async fn count_documents(&self) -> Result<u64> {
        let result = self.collection.estimated_document_count(None).await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn select_all() -> Result<()> {
        let client = SampleMongoClient::new("mongodb://root:password@localhost:27017", "dev").await?;
        let repository = ProductRepository::new(client, "product");
        let products = repository.select_all().await?;
        println!("{:?}", products);
        Ok(())
    }

    #[tokio::test]
    async fn select_by_id() -> Result<()> {
        let client = SampleMongoClient::new("mongodb://root:password@localhost:27017", "dev").await?;
        let repository = ProductRepository::new(client, "product");
        let product = repository.select_by_id(11).await?;
        println!("{:?}", product);
        Ok(())
    }

    #[tokio::test]
    async fn insert() -> Result<()> {
        let client = SampleMongoClient::new("mongodb://root:password@localhost:27017", "dev").await?;
        let repository = ProductRepository::new(client, "product");
        let product = Product::new(None, 100, "test".to_string(), 100, None);
        let result = repository.insert(product).await?;
        println!("{:?}", result);
        Ok(())
    }

    #[tokio::test]
    async fn insert_many() -> Result<()> {
        let client = SampleMongoClient::new("mongodb://root:password@localhost:27017", "dev").await?;
        let repository = ProductRepository::new(client, "product");
        let product1 = Product::new(None, 101, "test1".to_string(), 100, None);
        let product2 = Product::new(None, 102, "test2".to_string(), 100, None);
        let products = vec![product1, product2];
        let result = repository.insert_many(products).await?;
        println!("{:?}", result);
        Ok(())
    }

    #[tokio::test]
    async fn update_by_id() -> Result<()> {
        let client = SampleMongoClient::new("mongodb://root:password@localhost:27017", "dev").await?;
        let repository = ProductRepository::new(client, "product");
        let product = Product::new(None, 100, "ボールペン(青)".to_string(), 300, None);
        let result = repository.update_by_id(product).await?;
        println!("{:?}", result);
        Ok(())
    }

    #[tokio::test]
    async fn delete_by_id() -> Result<()> {
        let client = SampleMongoClient::new("mongodb://root:password@localhost:27017", "dev").await?;
        let repository = ProductRepository::new(client, "product");
        let result = repository.delete_by_id(100).await?;
        println!("{:?}", result);
        Ok(())
    }

    #[tokio::test]
    async fn count_documents() -> Result<()> {
        let client = SampleMongoClient::new("mongodb://root:password@localhost:27017", "dev").await?;
        let repository = ProductRepository::new(client, "product");
        let result = repository.count_documents().await?;
        println!("{:?}", result);
        Ok(())
    }
}
