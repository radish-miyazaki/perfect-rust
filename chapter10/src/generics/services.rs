use serde::de::DeserializeOwned;
use anyhow::Result;

use super::traits::{CsvReader, JsonReader};
use super::traits_impl::{CsvReaderImpl, JsonReaderImpl};

pub struct ReadService<T> {
    csv_reader: Box<dyn CsvReader<T>>,
    json_reader: Box<dyn JsonReader<T>>
}

impl<T: DeserializeOwned + 'static> ReadService<T> {
    pub fn new() -> Self {
        Self {
            csv_reader: Box::new(CsvReaderImpl::<T>::new()) as Box<dyn CsvReader<T>>,
            json_reader: Box::new(JsonReaderImpl::<T>::new()) as Box<dyn JsonReader<T>>
        }
    }

    pub fn csv_read(&self, file_path: &str) -> Result<Vec<T>> {
        self.csv_reader.read(file_path)
    }

    pub fn json_read(&self, file_path: &str) -> Result<Vec<T>> {
        self.json_reader.read(file_path)
    }
}
