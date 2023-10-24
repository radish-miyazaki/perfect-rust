use std::ops::Div;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::Builder;
use std::time::Duration;

pub struct Calculator;

impl Calculator {
    fn calc_sum(params: Arc<Mutex<Vec<u64>>>) -> u64 {
        let mut total = 0;

        {
            let values = params.lock().unwrap_or_else(|err| panic!("{:?}", err));
            println!("calc_sum: {:?}", params);
            for value in values.iter() {
                total = total + value;
                thread::sleep(Duration::from_secs(1));
                println!("{} value: {}", thread::current().name().unwrap(), total);
            }
        }

        println!("summary {:?}", params);
        total
    }

    fn calc_avg(params: Arc<Mutex<Vec<u64>>>) -> u64 {
        let mut avg = 0;
        let mut count = 0;

        {
            let values = params.lock().unwrap_or_else(|err| panic!("{:?}", err));
            println!("calc_avg: {:?}", params);
            count = values.iter().count() as u64;
            for value in values.iter() {
                avg = avg + value;
                thread::sleep(Duration::from_secs(1));
                println!("{} value: {}", thread::current().name().unwrap(), avg);
            }
        }

        println!("summary {:?}", params);
        avg.div(count)
    }

    pub fn use_mutex() {
        let values: Vec<u64> = vec![10, 20, 30, 40, 50];
        let params = Arc::new(Mutex::new(values));
        let mut handles = Vec::with_capacity(2);

        let _params = Arc::clone(&params);
        let builder = Builder::new().name(String::from("sum")).stack_size(1024 * 3);
        handles.push(builder.spawn(move || Self::calc_sum(_params)).unwrap());

        let builder = Builder::new().name(String::from("avg")).stack_size(1024 * 3);
        handles.push(builder.spawn(move || Self::calc_avg(Arc::clone(&params))).unwrap());

        for handle in handles {
            let thread = handle.thread().clone();
            let result = handle.join().unwrap();
            println!("{} result: {}", thread.name().unwrap(), result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn use_mutex_test() {
        Calculator::use_mutex();
    }
}


