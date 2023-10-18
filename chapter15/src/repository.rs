use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Repository<T, K, UPD> {
    async fn select_all(&self) -> Result<Vec<T>>;
    async fn select_by_id(&self, id: K) -> Result<T>;
    async fn insert(&self, entity: T) -> Result<UPD>;
    async fn insert_many(&self, entities: Vec<T>) -> Result<UPD>;
    async fn update_by_id(&self, entity: T) -> Result<UPD>;
    async fn delete_by_id(&self, id: K) -> Result<UPD>;
    async fn count_documents(&self) -> Result<u64>;
}
