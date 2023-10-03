#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("1st : {}", arr[0]);
    println!("Length : {}", arr.len());

    let mut idx = 0;
    loop {
        if arr[idx] % 2 == 0 {
            println!("Even : {}", arr[idx]);
            idx += 1;
        } else {
            println!("Odd : {}", arr[idx]);
            idx += 1;
            continue;
        }
        if arr[idx] == arr.len() {
            if arr[idx] % 2 == 0 {
                println!("Even : {}", arr[idx]);
                idx += 1;
            } else {
                println!("Odd : {}", arr[idx]);
                idx += 1;
                continue;
            }
            break;
        }
    }
}
