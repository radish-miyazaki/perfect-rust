#![allow(dead_code)]

use std::time::SystemTime;
use chrono::prelude::*;
use chrono_tz::Tz;

pub fn instantiate() {
    let now: DateTime<Utc> = Utc::now();
    println!("UTC time = {}", now);

    let now: DateTime<Local> = Local::now();
    println!("Local time = {}", now);
}

pub fn format() {
    let now: DateTime<Utc> = Utc::now();
    let format_date = now.format("%Y年%m月%d日").to_string();
    println!("{:?}", format_date);

    let now: DateTime<Local> = Local::now();
    let format_date_time = now.format("%Y年%m月%d日 %H時%M分%S秒").to_string();
    println!("{:?}", format_date_time);
}

pub fn from_string() {
    let rfc2822_type = DateTime::parse_from_rfc2822("Fri, 14 Jan 2022 10:52:37 +0200");
    println!("{}", rfc2822_type.unwrap());

    let rfc_3339_type = DateTime::parse_from_rfc3339("2022-01-14T10:52:37+02:00");
    println!("{}", rfc_3339_type.unwrap());

    let time_only = NaiveTime::parse_from_str("15:30:00", "%H:%M:%S");
    println!("{}", time_only.unwrap());

    let date_only = NaiveDate::parse_from_str("2022年01月14日", "%Y年%m月%d日");
    println!("{}", date_only.unwrap());

    let custom_format = NaiveDate::parse_from_str("10 2022 14", "%m %Y %d");
    println!("{}", custom_format.unwrap());
}

pub fn get() {
    let now = Utc::now();
    println!("y={}, m={}, d={}", now.year(), now.month(), now.day());
    println!("h={}, m={}, s={}, n={}",
             now.hour(), now.minute(), now.second(), now.nanosecond());

    let w = match &now.weekday() {
        Weekday::Mon => "月曜日",
        Weekday::Tue => "火曜日",
        Weekday::Wed => "水曜日",
        Weekday::Thu => "木曜日",
        Weekday::Fri => "金曜日",
        Weekday::Sat => "土曜日",
        Weekday::Sun => "日曜日",
    };
    println!("曜日 = {}", w);
}

pub fn time_zone() {
    let tokyo: DateTime<Tz> = Local::now().with_timezone(&chrono_tz::Asia::Tokyo);
    println!("Tokyo = {}", tokyo);

    let chicago: DateTime<Tz> = Local::now().with_timezone(&chrono_tz::America::Chicago);
    println!("Chicago = {}", chicago);

    let tokyo_n = tokyo.naive_local();
    let chicago_n = chicago.naive_local();
    println!("Tokyo = {}, Chicago = {}", tokyo_n, chicago_n);

    let duration: chrono::Duration = tokyo_n - chicago_n;
    println!("Diff hours = {}", duration.num_hours());
    println!("Diff seconds = {}", duration.num_seconds());
    println!("Diff nano seconds = {}", duration.num_nanoseconds().unwrap());
}

pub fn unix_epoch() {
    let x = Local::now().timestamp();
    println!("Local epoch = {}", x);

    let y = Utc::now().timestamp();
    println!("UTC epoch = {}", y);

    let z = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    println!("By SystemTime = {}", z.unwrap().as_secs());
}
