use postgres::NoTls;
use r2d2::PooledConnection;
use r2d2_postgres::PostgresConnectionManager;
use anyhow::Result;

use super::pool_1::SAMPLE_POOL1;

pub struct SamplePoolManager;

impl SamplePoolManager {
    pub fn client() -> Result<PooledConnection<PostgresConnectionManager<NoTls>>> {
        let pool = SAMPLE_POOL1.lock().unwrap();
        println!("state: {:?}", pool.state());

        let client = pool.get()?;
        Ok(client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::thread;

    use crate::section1::repository::Repository;
    use crate::section1::product_repository::ProductRepository;
    use crate::section1::transaction::TransactionUtil;

    #[test]
    fn use_connection_pool() -> Result<()> {
        let mut handlers = Vec::with_capacity(3);

        for num in 0..3 {
            handlers.push(thread::spawn(move || {
                let mut client = SamplePoolManager::client()?;
                let mut tx = TransactionUtil::start(&mut client, true)?;
                let mut repository = ProductRepository(&mut tx);
                repository.select_by_id(num + 1)
            }));
        }

        for handle in handlers {
            let result = handle.join().unwrap();
            println!("{}", result.unwrap().to_string());
        }

        Ok(())
    }
}
