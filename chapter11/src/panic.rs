#![allow(dead_code)]

use std::env::current_dir;
use std::fs::File;

pub fn use_except() {
    let file_path = current_dir()
        .map(|buf| buf.join("files/param.txt"))
        .map_err(|err| panic!("{}", err))
        .unwrap();

    File::open(file_path)
        .expect("Process exited: file not found");
}

pub fn use_panic() {
    let file_path = current_dir()
        .map(|buf| buf.join("files/param.txt"))
        .map_err(|err| panic!("{}", err))
        .unwrap();

    let file = File::open(file_path);
    if file.is_err() {
        panic!("{}, {:?}", "Process exited: file not found", file.err().unwrap());
    }
}
