use anyhow::Result;
use postgres::{Transaction, Client};

pub struct TransactionUtil;

impl TransactionUtil {
    pub fn start(client: &mut Client, read_only: bool) -> Result<Transaction> {
        let tx = client.build_transaction().read_only(read_only).start()?;
        Ok(tx)
    }

    pub fn commit(tx: Transaction) -> Result<()> {
        tx.commit()?;
        Ok(())
    }

    pub fn rollback(tx: Transaction) -> Result<()> {
        tx.rollback()?;
        Ok(())
    }
}
