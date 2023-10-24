use std::ops::Div;
use std::sync::Arc;
use crossbeam::channel::{bounded, Receiver, Sender};
use crossbeam::sync::ShardedLock;
use crossbeam::thread;

pub struct Calculator;

impl Calculator {
    fn create_data(sender: (Sender<()>, Sender<()>), params: Arc<ShardedLock<Vec<u64>>>)
        -> String
    {
        {
            let mut vals = params.write().unwrap_or_else(|err| panic!("{:?}", err));
            println!("{:?}", params);
            for num in 1..6 {
                vals.push(num * 100);
            }
        }

        sender.0.send(()).unwrap_or_else(|err| panic!("{:?}", err));
        sender.1.send(()).unwrap_or_else(|err| panic!("{:?}", err));
        String::from("finished to generate calculation data.")
    }

    fn calc_sum(receiver: Receiver<()>, params: Arc<ShardedLock<Vec<u64>>>) -> String {
        receiver.recv().unwrap_or_else(|err| panic!("{:?}", err));
        let vals = params.read().unwrap_or_else(|err| panic!("{:?}", err));

        let mut total = 0;
        for val in vals.iter() {
            total += val;
            std::thread::sleep(std::time::Duration::from_secs(1));
            println!("{} value: {}", std::thread::current().name().unwrap(), total);
        }
        total.to_string()
    }

    fn calc_avg(receiver: Receiver<()>, params: Arc<ShardedLock<Vec<u64>>>) -> String {
        receiver.recv().unwrap_or_else(|err| panic!("{:?}", err));
        let vals = params.read().unwrap_or_else(|err| panic!("{:?}", err));

        let mut avg = 0;
        for val in vals.iter() {
            avg += val;
            std::thread::sleep(std::time::Duration::from_secs(1));
            println!("{} value: {}", std::thread::current().name().unwrap(), avg);
        }
        avg.div(vals.len() as u64).to_string()
    }

    pub fn use_sharded_lock() {
        thread::scope(|scope| {
            let (s_sender, s_receiver) = bounded::<()>(5);
            let (a_sender, a_receiver) = bounded::<()>(5);

            let params = Arc::new(ShardedLock::new(Vec::<u64>::new()));
            let mut handles = Vec::with_capacity(3);

            let params_a = Arc::clone(&params);
            handles.push(scope.builder()
                .name(String::from("create_data"))
                .stack_size(1024 * 3)
                .spawn(|_| Self::create_data((s_sender, a_sender), params_a))
                .unwrap_or_else(|err| panic!("{:?}", err)));

            let params_b = Arc::clone(&params);
            handles.push(scope.builder()
                .name(String::from("calc_sum"))
                .stack_size(1024 * 3)
                .spawn(|_| Self::calc_sum(s_receiver, params_b))
                .unwrap_or_else(|err| panic!("{:?}", err)));

            let params_c = Arc::clone(&params);
            handles.push(scope.builder()
                .name(String::from("calc_avg"))
                .stack_size(1024 * 3)
                .spawn(|_| Self::calc_avg(a_receiver, params_c))
                .unwrap_or_else(|err| panic!("{:?}", err)));
        }).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn use_sharded_lock_test() {
        Calculator::use_sharded_lock();
    }
}
