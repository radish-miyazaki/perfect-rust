#![allow(dead_code)]

use crate::generics::services::ReadService;
use super::traits::{CsvReader, JsonReader};
use super::traits_impl::{CsvReaderImpl, JsonReaderImpl};
use super::entities::Product;

pub fn use_generics_method() {
    let csv_reader = CsvReaderImpl::<Product>::default();
    let json_reader = JsonReaderImpl::<Product>::default();

    let csv_path = concat!(env!("CARGO_MANIFEST_DIR"), "/data/products.csv");
    let json_path = concat!(env!("CARGO_MANIFEST_DIR"), "/data/products.json");

    let csv_result = csv_reader.read(csv_path).unwrap();
    let json_result = json_reader.read(json_path).unwrap();

    println!("<< CSV >>");
    for product in csv_result {
        println!("{:?}", product);
    }

    println!("<< JSON >>");
    for product in json_result {
        println!("{:?}", product);
    }
}

pub fn use_service_method() {
    let csv_path = concat!(env!("CARGO_MANIFEST_DIR"), "/data/products.csv");
    let json_path = concat!(env!("CARGO_MANIFEST_DIR"), "/data/products.json");

    let service = ReadService::<Product>::new();

    let csv_result = service.csv_read(csv_path).unwrap();
    let json_result = service.json_read(json_path).unwrap();

    println!("<< CSV >>");
    for product in csv_result {
        println!("{:?}", product);
    }

    println!("<< JSON >>");
    for product in json_result {
        println!("{:?}", product);
    }
}
