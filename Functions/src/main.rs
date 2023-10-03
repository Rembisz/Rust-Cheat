#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn hello() {
    println!("Hello"); // Create function
}

fn sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

fn sum2(x: i32, y: i32) -> i32 {
    // Returns type i32
    x + y // *no ;*
}

fn sum3(x: i32, y: i32) -> i32 {
    // Returns type i32
    return x + y; // *; included*
}

fn start_from(x: i32) -> (i32, i32, i32, i32, i32, i32, i32) {
    return (x, x + 1, x + 2, x + 3, x + 4, x + 5, x + 6);
}

fn sum_list(list: &[i32]) -> i32 {
    // References vector
    let mut sum = 0;
    for &val in list.iter() {
        sum += &val; // iterates vector and adds each value to i32 "sum"
    }
    sum // returns sum
}

fn main() {
    hello(); // Call function

    sum(5, 7);
    println!("{}", sum2(24, 95));
    println!("{}", sum3(24, 95));

    let (start, val1, val2, val3, val4, val5, val6) = start_from(3);
    println!(
        "Start is {start} : {} {} {} {} {} {} {}",
        start, val1, val2, val3, val4, val5, val6
    );

    let num_list = vec![1, 2, 3, 4, 5];
    println!("Sum of list = {}", sum_list(&num_list));
}
