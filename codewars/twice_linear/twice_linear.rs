// https://www.codewars.com/kata/5672682212c8ecf83e000050/train/rust

use std::io;
use std::mem;

mod my_lib;
use my_lib::*;

fn main() {
    let n = 10;
    println!("answer: {}", dbl_linear(n));
    //test();
}

fn add(mas: &mut [i32], n: usize) -> usize {
    let mut i: usize = 0;
    while (mas[i+1]>0) { i+=1; }
    mas[i+1] = 2 * mas[n] + 1;
    mas[i+2] = 3 * mas[n] + 1;
    i+2
}

fn dbl_linear(n: u32) -> u32{
    const ras: usize = 10000;
    let mut x: [i32;ras] = [0;ras];
    x[0] = 1;

    //let mut y: [i32;ras] =
    //analyze_slice(&mut x); // [0..ras]

    //test();
    let mut count: usize = 0;
    let mut trigger: usize = 0;
    let n: usize = n as usize;
    while (trigger<n) {
        trigger = add(&mut x, count);
        //println!("count: {}", count);
        count+=1;
    }

    //print_mas(&x);
    // delete "0"
    let mut i: usize = 1;
    while (x[i]>0) { i+=1; }
    let mut x = &mut x[0..i];
    //print_mas(&x);
    Qsort(&mut x);  // , qwerty: i32 = 0
    print_mas(&x);


    let a: u32 = x[0] as u32;
    a
    //x[0] // answer
}
