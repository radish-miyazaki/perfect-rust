#![allow(dead_code)]

use std::thread;
use std::thread::{Builder, JoinHandle};
use std::time::Duration;

fn summary_thread_1(name: String, values: Vec<u64>) -> JoinHandle<u64> {
    thread::spawn(move || {
        let mut total: u64 = 0;
        for value in values {
            total = total + value;
            thread::sleep(Duration::from_secs(1));
            println!("{}: {}", name, total);
        }

        total
    })
}

fn summary_thread_2(name: String, values: Vec<u64>) -> JoinHandle<u64> {
    let builder = Builder::new()
        .name(name)
        .stack_size(1024 * 3);

    builder.spawn(|| {
        let mut total: u64 = 0;
        for value in values {
            total = total + value;
            thread::sleep(Duration::from_secs(1));
            println!("{}: {}", thread::current().name().unwrap(), total);
        }

        total
    }).unwrap()
}

pub struct Summary;

impl Summary {
    pub fn summary_thread(&self, name: String, values: Vec<u64>) -> JoinHandle<u64> {
        let builder = Builder::new()
            .name(name)
            .stack_size(1024 * 3);

        builder.spawn(|| {
            let mut total: u64 = 0;
            for value in values {
                total = total + value;
                thread::sleep(Duration::from_secs(1));
                println!("{}: {}", thread::current().name().unwrap(), total);
            }

            total
        }).unwrap()
    }
}

pub fn thread_controller_1() {
    let thd1 = summary_thread_1(String::from("thd1"), vec![10, 20, 30, 40, 50]);
    let thd2 = summary_thread_1(String::from("thd2"), vec![100, 200, 300, 400, 500]);

    let res1 = thd1.join().map_err(|err| panic!("{:?}", err)).unwrap();
    let res2 = thd2.join().map_err(|err| panic!("{:?}", err)).unwrap();

    println!("Result 1: {}", res1);
    println!("Result 2: {}", res2);
}

pub fn thread_controller_2() {
    let thd1 = summary_thread_2(String::from("thd1"), vec![10, 20, 30, 40, 50]);
    let thd2 = summary_thread_2(String::from("thd2"), vec![100, 200, 300, 400, 500]);

    let res1 = thd1.join().map_err(|err| panic!("{:?}", err)).unwrap();
    let res2 = thd2.join().map_err(|err| panic!("{:?}", err)).unwrap();

    println!("Result 1: {}", res1);
    println!("Result 2: {}", res2);
}

pub fn thread_controller_3() {
    let summary = Summary;
    let thd1 = summary.summary_thread(String::from("thd1"), vec![10, 20, 30, 40, 50]);
    let thd2 = summary.summary_thread(String::from("thd2"), vec![100, 200, 300, 400, 500]);

    let res1 = thd1.join().map_err(|err| panic!("{:?}", err)).unwrap();
    let res2 = thd2.join().map_err(|err| panic!("{:?}", err)).unwrap();

    println!("Result 1: {}", res1);
    println!("Result 2: {}", res2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn thread_controller_1_test() {
        thread_controller_1();
    }

    #[test]
    pub fn thread_controller_2_test() {
        thread_controller_2();
    }

    #[test]
    pub fn thread_controller_3_test() {
        thread_controller_3();
    }
}
