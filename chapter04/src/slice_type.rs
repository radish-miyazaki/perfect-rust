#![allow(dead_code)]

pub fn get() {
    let str_array = ["ABC", "DEF", "GHI", "JKL", "MNO", "PQR", "STU"];
    let slice: &[&str] = &str_array[3..=5];
    let slice2 = &str_array[0..2];
    let slice3 = &str_array[..];

    println!("slice1 = {:?}", slice);
    println!("slice2 = {:?}", slice2);
    println!("slice3 = {:?}", slice3);
}

pub fn range() {
    let int_array = [1, 2, 3, 4, 5, 6, 7];
    let range = std::ops::Range { start: 1, end: 3 };
    let slice = &int_array[range];
    println!("slice = {:?}", slice);
}

pub fn multibyte_slice() {
    let company_name = String::from("合同会社ノマドリ");

    // 不正な範囲指定
    // let slice = &company_name[1..3];
    // println!("参照範囲 = {:?}, 大きさ = {}", slice, slice.len());

    let slice = &company_name[..12];
    println!("参照範囲 = {:?}, 大きさ = {}", slice, slice.len());
    let slice = &company_name[12..];
    println!("参照範囲 = {:?}, 大きさ = {}", slice, slice.len());
}

pub fn fat_pointer() {
    let int_array = [1, 2, 3, 4, 5, 6, 7];

    let slice: &[i32] = &int_array[0..];
    println!("参照範囲 = {:?}, ポインタ = {:p}, 大きさ = {}", slice, slice, slice.len());

    let slice = &int_array[3..5];
    println!("参照範囲 = {:?}, ポインタ = {:p}, 大きさ = {}", slice, slice, slice.len());
}

pub fn methods_1() {
    let array = [100, 101, 102, 103, 104];
    let slice: &[i32] = &array[0..];
    println!("first() = {:?}", slice.first());
    println!("last() = {:?}", slice.last());
    println!("get(2) = {:?}", slice.get(2));
    println!("is_empty() = {}", slice.is_empty());
    println!("len() = {}", slice.len());
}

pub fn methods_2() {
    let mut array = [103, 101, 100, 104, 102];
    let slice: &mut [i32] = &mut array[..];

    slice.reverse();
    println!("reverse() = {:?}", slice);

    slice.sort();
    println!("sort() = {:?}", slice);
}

pub fn methods_3() {
    let vec = vec!["abc", "def", "hij", "rst", "uvw", "xyz"];
    let slice = &vec[..];
    let chks = slice.chunks(2);
    for chk in chks {
        println!("chunk = {:?}", chk);
    }

    let rchks = slice.rchunks(2); // Iter[["hij", "rst"], ["abc", "def"]]
    for rchk in rchks {
        println!("rchunks = {:?}", rchk);
    }

    let r = slice.join("/");
    println!("join() = {}", r);

    let it = slice.iter();
    println!("iter() = {:?}", it);

    let v = slice.to_vec();
    println!("to_vec() = {:?}", v);

    let array = [100, 101, 102, 103, 104, 105];
    let slice: &[i32] = &array[..];
    let spts = slice.split(|val| {
        val % 4 == 0
    });
    for spt in spts {
        println!("split = {:?}", spt);
    }

    let rspts = slice.rsplit(|val| {
        val % 4 == 0
    });
    // Iter[[], [101, 102, 103], [105]
    for rspt in rspts {
        println!("rsplit = {:?}", rspt);
    }
}
