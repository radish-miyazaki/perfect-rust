use serde::de::DeserializeOwned;
use anyhow::Result;
use crate::generics::traits_impl::CsvReaderImpl;

pub trait CsvReader<T>: Send + Sync where T: DeserializeOwned {
    fn read(&self, file_path: &str) -> Result<Vec<T>>;
}

impl<T> CsvReader<T> for CsvReaderImpl<T> where Self: Send + Sync, T: DeserializeOwned {
    fn read(&self, file_path: &str) -> Result<Vec<T>> {
        todo!()
    }
}
