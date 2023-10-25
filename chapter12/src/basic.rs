#![allow(dead_code)]

use std::sync::{Arc, Barrier};
use std::thread;
use std::thread::{Builder, current, JoinHandle};
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
    fn summary(name: &str, values: Vec<u64>) -> u64 {
        let mut total = 0;
        for value in values {
            total = total + value;
            println!("{}: {}", name, total);
        }
        total
    }

    pub fn summary_thread(&self, name: String, values: Vec<u64>) -> JoinHandle<u64> {
        let builder = Builder::new()
            .name(name)
            .stack_size(1024 * 3);

        builder.spawn(|| {
            Self::summary(thread::current().name().unwrap(), values)
        }).unwrap()
    }

    fn summary_a(values: Vec<u64>) -> u64 {
        let mut total = 0;
        for value in values {
            total = total + value;
        }
        total
    }

    pub fn use_barrier(&self) {
        let mut handles = Vec::with_capacity(3);
        let barrier = Arc::new(Barrier::new(3));
        let mut num = 0;

        while num <= 2 {
            let arc = Arc::clone(&barrier);
            handles.push(
                Builder::new()
                    .name(format!("summary{}", num))
                    .stack_size(1024 * 5)
                    .spawn(move || {
                        let data: Vec<u64> = vec![10+num, 20+num, 30+num, 40+num, 50+num];
                        let result = Self::summary_a(data);
                        let wresult = arc.wait();
                        println!("{} ended: {}", current().name().unwrap(), wresult.is_leader());
                        result
                    }).unwrap_or_else(|err| panic!("{:?}", err)));
            num += 1;
        }

        for handle in handles {
            let thread = handle.thread().clone();
            let result = handle.join().unwrap_or_else(|err| panic!("{:?}", err));
            println!("{} sum: {}", thread.name().unwrap(), result);
        }
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

pub fn thread_controller_4() {
    let summary = Summary;
    summary.use_barrier();
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

    #[test]
    pub fn thread_controller_4_test() {
        thread_controller_4();
    }
}
