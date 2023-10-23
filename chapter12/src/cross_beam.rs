#![allow(dead_code)]

use std::thread::current;
use crossbeam::sync::WaitGroup;
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

    pub fn use_wait_group(&self) {
        thread::scope(|scope| {
            let mut handles = Vec::with_capacity(3);
            let wait_group = WaitGroup::new();
            let mut num = 0;

            while num <= 2 {
                let wg = wait_group.clone();
                handles.push(
                    scope.builder()
                        .name(format!("summary{}", num))
                        .stack_size(1024 * 3)
                        .spawn(move |_| {
                            let result = self.summary(
                                current().name().unwrap(),
                                vec![10+num, 20+num, 30+num, 40+num, 50+num]);
                            drop(wg);
                            result
                        }).unwrap_or_else(|err| panic!("{:?}", err)));

                num += 1;
            }
            wait_group.wait();

            for handle in handles {
                let thread = handle.thread().clone();
                let result = handle.join().unwrap_or_else(|err| panic!("{:?}", err));
                println!("{} sum: {}", thread.name().unwrap(), result);
            }
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

pub fn thread_controller_3() {
    let s = Summary;
    s.use_wait_group();
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

