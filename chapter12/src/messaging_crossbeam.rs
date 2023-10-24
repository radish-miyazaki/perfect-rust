use crossbeam::channel::{bounded, Receiver, Sender};
use crossbeam::thread;

use super::station::*;

pub struct Client;

impl Client {
    pub fn search_request(p_sender: Sender<String>, c_receiver: Receiver<String>) {
        loop {
            let entry_name = enter_station();
            p_sender.send(entry_name.clone()).unwrap_or_else(|err| panic!("{:?}", err));
            if entry_name == "end" {
                break;
            }

            c_receiver.recv().and_then(|result| {
                println!("{:?}", result);
                Ok(())
            }).unwrap_or_else(|err| panic!("{:?}", err));
        }

        print!("Finished Client.")
    }
}

pub struct Provider;

impl Provider {
    pub fn search_service(c_sender: Sender<String>, p_receiver: Receiver<String>) {
        loop {
            let entry_name = p_receiver.recv().unwrap_or_else(|err| panic!("{:?}", err));
            if entry_name == "end" {
                break;
            }

            let result = match search_station(entry_name) {
                Some(ret) => ret,
                None => String::from("Not found.")
            };
            c_sender.send(result).unwrap_or_else(|err| panic!("{:?}", err));
        }

        println!("Finished Provider.");
    }
}

pub fn execute() {
    let (p_sender, p_receiver) = bounded::<String>(5);
    let (c_sender, c_receiver) = bounded::<String>(5);

    thread::scope(|scope| {
        let p_handle = scope.spawn(|_| {
            Provider::search_service(c_sender, p_receiver);
        });

        let c_handle = scope.spawn(|_| {
            Client::search_request(p_sender, c_receiver);
        });

        p_handle.join().unwrap_or_else(|err| panic!("{:?}", err));
        c_handle.join().unwrap_or_else(|err| panic!("{:?}", err));
    }).unwrap();
}
