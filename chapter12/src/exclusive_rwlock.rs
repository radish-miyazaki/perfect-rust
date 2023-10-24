use std::ops::Div;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, mpsc, RwLock};
use std::thread::Builder;
use std::time::Duration;
use anyhow::Result;

pub struct Calculator;

impl Calculator {
    fn create_data(sender: (Sender<()>, Sender<()>), params: Arc<RwLock<Vec<u64>>>)
        -> Result<String>
    {
        {
            let mut vals = params.write().unwrap();
            println!("{:?}", params);
            for num in 1..6 {
                vals.push(num);
            }
        }

        // 送信側のスレッドを起動
        sender.0.send(()).unwrap_or_else(|err| panic!("{:?}", err));
        sender.1.send(()).unwrap_or_else(|err| panic!("{:?}", err));
        Ok(String::from("finished to generate calculation data."))
    }

    fn calc_sum(receiver: Receiver<()>, params: Arc<RwLock<Vec<u64>>>) -> Result<String> {
        receiver.recv().unwrap_or_else(|err| panic!("{:?}", err));
        let vals = params.read().unwrap();

        let mut total = 0;
        for val in vals.iter() {
            total += val;
            std::thread::sleep(std::time::Duration::from_secs(1));
            println!("{} value: {}", std::thread::current().name().unwrap(), total);
        }

        Ok(total.to_string())
    }

    fn calc_avg(receiver: Receiver<()>, params: Arc<RwLock<Vec<u64>>>) -> Result<String> {
        receiver.recv().unwrap_or_else(|err| panic!("{:?}", err));
        let vals = params.read().unwrap();

        let mut avg = 0;
        for val in vals.iter() {
            avg += val;
            std::thread::sleep(Duration::from_secs(1));
            println!("{} value: {}", std::thread::current().name().unwrap(), avg);
        }

        Ok(avg.div(vals.len() as u64).to_string())
    }

    pub fn use_rwlock() {
        let (s_sender, s_receiver) = mpsc::channel::<()>();
        let (a_sender, a_receiver) = mpsc::channel::<()>();

        let params = Arc::new(RwLock::new(Vec::<u64>::new()));
        let mut handles = Vec::with_capacity(3);

        let builder = Builder::new()
            .name(String::from("create_data"))
            .stack_size(1024 * 3);
        let params_a = Arc::clone(&params);
        handles.push(
            builder.spawn(move || {
                Self::create_data((s_sender, a_sender), params_a)
            }).unwrap_or_else(|err| panic!("{:?}", err)));

        let builder = Builder::new()
            .name(String::from("sum"))
            .stack_size(1024 * 3);
        let params_b = Arc::clone(&params);
        handles.push(
            builder.spawn(move || {
                Self::calc_sum(s_receiver, params_b)
            }).unwrap_or_else(|err| panic!("{:?}", err)));

        let builder = Builder::new()
            .name(String::from("avg"))
            .stack_size(1024 * 3);
        let params_c = Arc::clone(&params);
        handles.push(
            builder.spawn(move || {
                Self::calc_avg(a_receiver, params_c)
            }).unwrap_or_else(|err| panic!("{:?}", err)));

        for handle in handles {
            let thread = handle.thread().clone();
            let result = handle.join().unwrap_or_else(|err| panic!("{:?}", err));
            println!("{} result: {:?}", thread.name().unwrap(), result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_rwlock() {
        Calculator::use_rwlock();
    }
}
