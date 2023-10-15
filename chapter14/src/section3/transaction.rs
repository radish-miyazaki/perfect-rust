use anyhow::Result;

use tokio_postgres::{Client, Transaction};

pub struct AsyncTransaction;

impl AsyncTransaction {
    pub async fn start(client: &mut Client, read_only: bool) -> Result<Transaction> {
        let tx = client.build_transaction().read_only(read_only).start().await?;
        Ok(tx)
    }
}
