use anyhow::Result;
use postgres::{Client, Config, NoTls};

use super::params::ConnectParams;

pub struct PostgresSampleClient;

impl PostgresSampleClient {
    pub fn simple_connect(params: ConnectParams) -> Result<Client> {
        let client = Client::connect(params.connect_string().as_str(), NoTls)?;
        Ok(client)
    }

    pub fn config_connect(params: ConnectParams) -> Result<Client> {
        let client = Config::new()
            .host(params.get_host())
            .port(params.get_port().clone())
            .dbname(params.get_dbname())
            .user(params.get_user())
            .password(params.get_password())
            .connect(NoTls)?;
        Ok(client)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_ok() {
        let params = ConnectParams::new(
            "localhost".to_string(),
            5432,
            "product".to_string(),
            "admin".to_string(),
            "password".to_string()
        );

        match PostgresSampleClient::simple_connect(params.clone()) {
            Ok(client) => {
                println!("simple_connect: success");
                let _ = client.close();
            },
            Err(err) => println!("{:?}", err.to_string())
        };

        match PostgresSampleClient::config_connect(params.clone()) {
            Ok(client) => {
                println!("config_connect: success");
                let _ = client.close();
            },
            Err(err) => println!("{:?}", err.to_string())
        };
    }
}
