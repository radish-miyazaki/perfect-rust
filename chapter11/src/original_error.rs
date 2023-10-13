#![allow(dead_code)]

use std::error::Error;
use std::fmt::{Debug, Display};
use std::num::{ParseFloatError, ParseIntError};
use std::str::FromStr;

use num_traits::NumOps;

#[derive(Debug)]
pub enum SampleError {
    IntError(ParseIntError),
    FloatError(ParseFloatError)
}

impl Error for SampleError {}

impl Display for SampleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SampleError::IntError(e) => write!(f, "ParseIntError: {}", e),
            SampleError::FloatError(e) => write!(f, "ParseFloatError: {}", e)
        }
    }
}

impl From<ParseIntError> for SampleError {
    fn from(e: ParseIntError) -> Self {
        SampleError::IntError(e)
    }
}

impl From<ParseFloatError> for SampleError {
    fn from(e: ParseFloatError) -> Self {
        SampleError::FloatError(e)
    }
}


fn parse_02<T>(val: String) -> Result<T, SampleError>
where
    T: NumOps + FromStr,
    SampleError: From<<T as FromStr>::Err>
{
    val.parse::<T>().map_err(|e| SampleError::from(e))
}

pub fn use_parse_02() {
    let res = parse_02::<i32>(String::from("123")).unwrap();
    println!("{}", res);
    let res = parse_02::<i32>(String::from("ABC")).err().unwrap();
    println!("{:?}", res);

    let res = parse_02::<f32>(String::from("123")).unwrap();
    println!("{}", res);
    let res = parse_02::<f32>(String::from("ABC")).err().unwrap();
    println!("{:?}", res);
}
