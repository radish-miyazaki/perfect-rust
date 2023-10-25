#![allow(dead_code)]

use anyhow::Result;
use async_std::channel::unbounded;
use async_std::task::{JoinHandle, spawn};
use crate::client::Client;

use crate::customer::Customer;
use crate::writer::Writer;

pub async fn controller() {
    let (csv_sender, csv_receiver) = unbounded::<Customer>();
    let (json_sender, json_receiver) = unbounded::<Customer>();

    let mut handles = Vec::<JoinHandle<Result<String>>>::new();
    handles.push(
        spawn(async {
            let wtr = Writer;
            wtr.csv_writer(csv_receiver).await
        })
    );
    handles.push(
        spawn(async {
            let wtr = Writer;
            wtr.json_writer(json_receiver).await
        })
    );
    handles.push(
        spawn(async {
            let client = Client;
            client.entry((csv_sender, json_sender)).await
        })
    );

    for handle in handles {
        match handle.await {
            Ok(result) => println!("{}", result),
            Err(err) => println!("{:?}", err)
        }
    }
}
