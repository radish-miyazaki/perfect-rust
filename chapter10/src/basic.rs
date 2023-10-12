#![allow(dead_code)]

use anyhow::Result;

pub trait Calculator {
    fn calc(&self) -> Result<u64> {
        todo!("Not implemented yet")
    }
}

pub struct Rectangle {
    width: u64,
    height: u64,
}

impl Calculator for Rectangle {
    fn calc(&self) -> Result<u64> {
        Ok(self.width * self.height)
    }
}

pub fn use_rectangle() {
    let r = Rectangle{ width: 100, height: 50 };
    let result = r.calc();
    println!("Area = {}", result.unwrap());
}
