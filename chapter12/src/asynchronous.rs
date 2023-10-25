#![allow(dead_code)]

use std::time::Duration;
use async_std::task;
use async_std::task::Builder;
use async_std::task::block_on;

async fn calc_sum(vals: Vec<u64>) -> u64 {
    let mut total = 0;
    for val in vals {
        total = total + val;
        std::thread::sleep(Duration::from_secs(1));
        println!("total = {}", total)
    }
    total
}

pub fn use_calc_sum() {
    block_on(async {
        let total = calc_sum(vec![1, 2, 3, 4, 5]).await;
        println!("total = {}", total);
    })
}

pub async fn use_calc_sum_async() {
    let total = calc_sum(vec![1, 2, 3, 4, 5]).await;
    println!("total = {}", total);
}

pub async fn use_spawn() {
    let handle1 = task::spawn(async {
        calc_sum(vec![10, 20, 30, 40, 50, 60 ,70, 80, 90, 100]).await
    });

    let handle2 = task::spawn(async {
        calc_sum(vec![10, 20, 30, 40, 50]).await
    });

    async {
        for _ in 0..5 {
            println!("Not task processing");
            std::thread::sleep(Duration::from_secs(1));
        }
    }.await;

    println!("sum1: {}", handle1.await);
    println!("sum2: {}", handle2.await);
}

pub async fn use_builder() {
    let task1 = Builder::new().name(String::from("task1"))
        .local(async { calc_sum(vec![10, 20, 30, 40, 50]).await });
    let task2 = Builder::new().name(String::from("task2"))
        .local(async { calc_sum(vec![100, 200, 300, 400, 500]).await });

    match task1 {
        Ok(handle) => println!("task1: {}", handle.await),
        Err(err) => println!("task1: {:?}", err)
    }

    match task2 {
        Ok(handle) => println!("task2: {}", handle.await),
        Err(err) => println!("task2: {:?}", err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn use_calc_sum_test() {
        use_calc_sum();
    }

    #[test]
    pub fn use_calc_sum_async_test() {
        block_on(use_calc_sum_async());
    }

    #[test]
    pub fn use_spawn_test() {
        block_on(use_spawn());
    }

    #[test]
    pub fn use_builder_test() {
        block_on(use_builder());
    }
}
