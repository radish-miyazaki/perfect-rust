use anyhow::Result;
use lombok::*;
use mongodb::{Client, Database};

#[derive(Debug, Getter, GetterMut)]
pub struct SampleMongoClient {
    client: Client,
    #[allow(dead_code)]
    database: Database
}

impl SampleMongoClient {
    pub async fn new(uri: &str, name: &str) -> Result<Self> {
        let client = Client::with_uri_str(uri).await?;
        let database = client.database(name);
        Ok(Self { client, database })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn new() -> Result<()> {
        let result = SampleMongoClient::new("mongodb://localhost:27017", "dev").await;
        match result {
            Ok(sample_client) => {
                println!("{:?}", sample_client.get_client());
            },
            Err(err) => {
                println!("{:?}", err);
                assert!(false)
            }
        }

        Ok(())
    }
}
