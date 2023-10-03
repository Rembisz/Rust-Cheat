#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut idx = 0;
    while idx < arr.len() {
        println!("While Looped Array: {}", arr[idx]);
        idx += 1;
    }

    idx = 0;
    for idx in arr.iter() {
        println!("For Looped Array: {}", idx);
    }
}
