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

fn parse_03<T>(val: String) -> Result<T, SampleError>
where
    T: NumOps + FromStr,
    SampleError: From<<T as FromStr>::Err>
{
    val.parse::<T>().map_err(|e| SampleError::from(e))
}

pub fn use_parse_03() {
    let res = parse_03::<i32>(String::from("123")).unwrap();
    println!("{}", res);
    let res = parse_03::<i32>(String::from("ABC")).err().unwrap();
    println!("{:?}", res.to_string());

    let res = parse_03::<f32>(String::from("123")).unwrap();
    println!("{}", res);
    let res = parse_03::<f32>(String::from("ABC")).err().unwrap();
    println!("{:?}", res.to_string());
}
