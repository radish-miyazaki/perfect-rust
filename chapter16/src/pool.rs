use std::env;
use std::time::Duration;

use anyhow::Result;
use dotenv::dotenv;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub struct SamplePool;

impl SamplePool {
    pub async fn get() -> Result<DatabaseConnection> {
        dotenv().ok();
        let uri = env::var("DATABASE_URL")?;
        let mut opts = ConnectOptions::new(uri);
        opts.max_connections(10)
            .min_connections(5)
            .idle_timeout(Duration::from_secs(10))
            .max_lifetime(Duration::from_secs(10))
            .sqlx_logging(true);

        let conn = Database::connect(opts).await?;
        Ok(conn)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get() -> Result<()> {
        let conn = SamplePool::get().await.unwrap();
        match conn.ping().await {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow::anyhow!(e)),
        }
    }
}
