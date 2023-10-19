use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait Repository {
    type Transaction;
    type Model;
    type ActiveModel;

    async fn select_all(&self, tx: &Self::Transaction) -> Result<Vec<Self::Model>>;
    async fn select_by_id(&self, tx: &Self::Transaction, id: i64) -> Result<Self::Model>;
    async fn select_by_name_like(&self, tx: &Self::Transaction, keyword: &str) -> Result<Vec<Self::Model>>;
    async fn insert(&self, tx: Self::Transaction, row: Self::ActiveModel) -> Result<Self::Model>;
    async fn update_by_id(&self, tx: Self::Transaction, row: Self::ActiveModel) -> Result<Self::Model>;
    async fn delete_by_id(&self, tx: Self::Transaction, id: i64) -> Result<u64>;
}
