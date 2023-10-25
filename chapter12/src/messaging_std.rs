#![allow(dead_code)]

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
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
    let (c_sender, c_receiver) = mpsc::channel::<String>();
    let (p_sender, p_receiver) = mpsc::channel::<String>();

    let p_handle = thread::spawn(move || {
        Provider::search_service(c_sender, p_receiver);
    });
    let c_handle = thread::spawn(move || {
        Client::search_request(p_sender, c_receiver);
    });

    p_handle.join().unwrap_or_else(|err| panic!("{:?}", err));
    c_handle.join().unwrap_or_else(|err| panic!("{:?}", err));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn execute_test() {
        execute();
    }
}

