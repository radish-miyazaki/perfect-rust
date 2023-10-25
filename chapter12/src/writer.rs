use std::env;
use std::fs::File;
use anyhow::Result;
use async_std::channel::Receiver;
use csv::WriterBuilder;

use crate::customer::Customer;

pub struct Writer;

impl Writer {
    pub async fn csv_writer(&self, receiver: Receiver<Customer>)
        -> Result<String>
    {
        let mut customers: Vec<Customer> = Vec::new();

        loop {
            let customer = receiver.recv().await?;
            if customer.name() == "end" {
                break;
            }
            customers.push(customer);
        }

        let path = env::current_dir().map(|p| p.join("resources/customers.csv"))?;
        let mut wtr = WriterBuilder::new().from_path(path)?;
        for customer in customers {
            wtr.serialize(customer)?;
        }

        Ok("CSV writer end.".to_string())
    }

    pub async fn json_writer(&self, receiver: Receiver<Customer>)
        -> Result<String>
    {
        let mut customers: Vec<Customer> = Vec::new();

        loop {
            let customer = receiver.recv().await?;
            if customer.name() == "end" {
                break;
            }
            customers.push(customer);
        }

        let path = env::current_dir().map(|p| p.join("resources/customers.json"))?;
        let wtr = File::create(path).map(|f| std::io::BufWriter::new(f))?;
        serde_json::to_writer_pretty(wtr, &customers)?;

        Ok("JSON writer end.".to_string())
    }
}
