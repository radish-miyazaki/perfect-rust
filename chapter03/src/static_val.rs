#![allow(dead_code)]
static TAX_RATE: f32 = 0.10;

#[allow(dead_code)]
pub fn calc_amount(price: i32) {
    let fprice = price as f32;
    let result = fprice + fprice * TAX_RATE;
    println!("単価:{}の税込金額={}", price, result as i32);
}

static mut TOTAL_VALUE: i32 = 0;

pub fn calc_total(value: i32) {
    unsafe {
        TOTAL_VALUE += value;
        println!("TOTAL_VALUE = {}", TOTAL_VALUE);
    }
}
