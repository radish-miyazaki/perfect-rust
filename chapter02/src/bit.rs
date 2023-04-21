#[allow(dead_code)]
pub fn use_symbol(x: i8, y: i8) {
    println!("x & y = {:b}", x & y);
    println!("x | y = {:b}", x | y);
    println!("x ^ y = {:b}", x ^ y);
    println!("!x = {:b}", !x);
    println!("x >> 3 = {:b}", x >> 3);
    println!("y << 3 = {:b}", y << 3);
}

#[allow(dead_code)]
pub fn use_format_specifier() {
    let x = 127;
    println!("2進数: {:b}", x);
    println!("8進数: {:o}", x);
    println!("16進数: {:x}", x);
    println!("指数: {:e}", x);
    println!("指数: {:E}", x);
}

#[allow(dead_code)]
pub fn use_method(x: i8, y: i8) {
    use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};
    println!("x & y = {:b}", x.bitand(y));
    println!("x | y = {:b}", x.bitor(y));
    println!("x ^ y = {:b}", x.bitxor(y));
    println!("!x = {:b}", x.not());
    println!("x >> 3 = {:b}", x.shr(3));
    println!("y << 3 = {:b}", y.shl(3));
}

#[allow(dead_code)]
pub fn compound_assign(mut x: i32, mut y: i32) {
    x &= y;
    println!("x &= y = {:b}", x);
    x |= y;
    println!("x |= y = {:b}", x);
    x >>= 3;
    println!("x >>= 3 = {:b}", x);
    y <<= 3;
    println!("y <<= 3 = {:b}", y);
    x ^= y;
    println!("x ^= y = {:b}", x);
}

#[allow(dead_code)]
pub fn compound_assign_method(mut x: i32, mut y: i32) {
    use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};
    x.bitand_assign(y);
    println!("x &= y = {:b}", x);
    x.bitor_assign(y);
    println!("x |= y = {:b}", x);
    x.shr_assign(3);
    println!("x >>= 3 = {:b}", x);
    y.shl_assign(3);
    println!("y <<= 3 = {:b}", y);
    x.bitxor_assign(y);
    println!("x ^= y = {:b}", x);
}
