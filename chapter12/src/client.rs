use anyhow::Result;
use async_std::channel::Sender;

use crate::customer::Customer;

pub struct Client;

impl Client {
    pub async fn entry(&self, sender: (Sender<Customer>, Sender<Customer>))
        -> Result<String>
    {
        loop {
            let name = enter("Enter customer name.");
            let email = if name == "end" {
                "end".to_string()
            } else {
                enter("Enter customer email.")
            };

            let customer = Customer::new(name.clone(), email.clone());
            sender.0.send(customer.clone()).await?;
            sender.1.send(customer.clone()).await?;

            if name == "end" {
                break;
            }
        }

        Ok("Client end.".to_string())
    }
}

pub fn enter(msg: &str) -> String {
    println!("{}", msg);

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    input.trim().to_string()
}
