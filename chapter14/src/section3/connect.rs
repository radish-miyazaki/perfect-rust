use std::sync::Mutex;

use once_cell::sync::Lazy;
use anyhow::Result;
use tokio_postgres::{Client, NoTls};

use crate::section1::params::ConnectParams;

pub static CONNECT_PARAMS: Lazy<Mutex<ConnectParams>> =
    Lazy::new(|| {
        let params = ConnectParams::new(
            "localhost".to_string(),
            5432,
            "product".to_string(),
            "admin".to_string(),
            "password".to_string()
        );

        Mutex::new(params)
    });

pub struct AsyncSimpleClient;

impl AsyncSimpleClient {
    pub async fn connect() -> Result<Client> {
        let params = CONNECT_PARAMS.lock().unwrap();
        let (client, connection) = tokio_postgres::connect(
            params.connect_string().as_str(),
            NoTls
        ).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                panic!("Connection error: {}", e);
            }
        });

        Ok(client)
    }
}
