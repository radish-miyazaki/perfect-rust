#![allow(dead_code)]

use std::sync::Mutex;

use once_cell::sync::Lazy;
use postgres::NoTls;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

static CONNECT_STR: &str = "host=localhost port=5432 dbname=product user=admin password=password";

pub static SAMPLE_POOL1: Lazy<Mutex<Pool<PostgresConnectionManager<NoTls>>>> = Lazy::new(|| {
    let config = CONNECT_STR.parse().unwrap();
    let manager = PostgresConnectionManager::new(config, NoTls);
    let pool = Pool::new(manager).unwrap();
    Mutex::new(pool)
});
