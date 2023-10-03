#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::Add; // Required to add generics

fn sum_gen<AnyType: Add<Output = AnyType>>(x: AnyType, y: AnyType) -> AnyType {
    return x + y; // References 2 generics (AnyType) and adds them together
}

fn main() {
    println!("5 + 4 = {}", sum_gen(5, 4));
    println!("5.2 + 4.9 = {}", sum_gen(5.2, 4.9));
}
