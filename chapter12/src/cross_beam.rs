#![allow(dead_code)]

use std::thread::current;
use crossbeam::thread;

pub struct Summary;

impl Summary {
    fn summary(&self, name: &str, values: Vec<u64>) -> u64 {
        let mut total = 0;
        for value in values {
            total = total + value;
            println!("{}: {}", name, total);
        }
        total
    }

    pub fn summary_thread(&self) {
        thread::scope(|scope| {
            let handle1 = scope.spawn(|_| self.summary("thd1", vec![10, 20, 30, 40, 50]));
            let handle2 = scope.spawn(|_| self.summary("thd2", vec![100, 200, 300, 400, 500]));

            let total1 = handle1.join().unwrap_or_else(|err| panic!("{:?}", err));
            let total2 = handle2.join().unwrap_or_else(|err| panic!("{:?}", err));

            println!("total1: {}, total2: {}", total1, total2);
        }).unwrap();
    }

    pub fn use_builder(&self) {
        thread::scope(|scope| {
            let handle = scope.builder()
                .name(String::from("sum1"))
                .stack_size(1024 * 3)
                .spawn(|_| {
                    self.summary(current().name().unwrap(), vec![10, 20, 30, 40, 50])
                }).unwrap_or_else(|err| panic!("{:?}", err));

            let total = handle.join().unwrap_or_else(|err| panic!("{:?}", err));

            println!("total: {}", total)
        }).unwrap();
    }
}

pub fn thread_controller_1() {
    let s = Summary;
    s.summary_thread();
}

pub fn thread_controller_2() {
    let s = Summary;
    s.use_builder();
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
}

