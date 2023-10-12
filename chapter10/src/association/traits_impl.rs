use std::fs::{File, read_to_string};
use std::io::BufReader;
use std::marker::PhantomData;
use std::path::PathBuf;

use anyhow::Result;
use csv::ReaderBuilder;
use serde::de::DeserializeOwned;

use super::traits::{CsvReader, JsonReader};

#[derive(Default)]
pub struct CsvReaderImpl<T>{
    phantom: PhantomData<T>
}

impl<T> CsvReader for CsvReaderImpl<T> where T: DeserializeOwned {
    type Entity = T;

    fn read(&self, file_path: &str) -> Result<Vec<Self::Entity>> {
        let path_buf = PathBuf::from(file_path);
        let string_data = read_to_string(path_buf)?;

        // バイナリ形式に変換
        let mut reader = ReaderBuilder::new().delimiter(b',').from_reader(string_data.as_bytes());

        let rows = reader.deserialize::<T>();
        let mut result = Vec::<T>::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }
}

#[derive(Default)]
pub struct JsonReaderImpl<T>{
    phantom: PhantomData<T>
}

impl<T> JsonReader for JsonReaderImpl<T> where T: DeserializeOwned {
    type Entity = T;

    fn read(&self, file_path: &str) -> Result<Vec<Self::Entity>> {
        let path_buf = PathBuf::from(file_path);

        let buf_reader = File::open(path_buf).map(|f| BufReader::new(f))?;

        let result = serde_json::from_reader(buf_reader)?;
        Ok(result)
    }
}
