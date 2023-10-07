#![allow(dead_code)]

// pub fn reference_1() {
//     let x = Vec::<i32>::new();
//     let y = &mut x;
//
//     y.push(1);
//     y.push(2);
//     y.push(3);
//     println!("{:?}", y);
// }

pub fn reference_2() {
    let mut x = Vec::<i32>::new();
    let y = &mut x;

    y.push(1);
    y.push(2);
    y.push(3);
    println!("{:?}", y);
}

// pub fn reference_3() {
//     let mut x = Vec::<i32>::new();
//     let y = &mut x;
//     let z = &mut x;
//
//     y.push(100);
//     z.push(200);
//     println!("{:?}", x);
// }

pub fn reference_4() {
    let mut x = Vec::<i32>::new();
    let y = &mut x;
    // let z = &x;
    y.push(100);
    println!("{:?}", x);
}
