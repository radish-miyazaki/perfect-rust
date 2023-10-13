#![allow(dead_code)]

use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

use num_traits::NumOps;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SampleError {
    #[error(transparent)]
    IntError(#[from] ParseIntError),
    #[error(transparent)]
    FloatError(#[from] ParseFloatError),
}

type SampleResult<T> = anyhow::Result<T, anyhow::Error>;

fn parse_04<T>(val: String) -> SampleResult<T>
where
    T: NumOps + FromStr,
    SampleError: From<<T as FromStr>::Err>
{
    val.parse::<T>().map_err(|e| {
        let context = format!("Specified value: {} is not parsable", val);
        let err = SampleError::from(e);
        anyhow::Error::new(err).context(context)
    })
}

pub fn use_parse_04() {
    let result = parse_04::<i32>(String::from("ABC")).err().unwrap();
    println!("{:?}", result);
    println!("{:?}", result.source().unwrap());
}

