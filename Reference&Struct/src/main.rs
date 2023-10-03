#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

struct Foo {
    x: String,
    x1: i32,
    x2: i32,
    x3: i32,
    x4: i32,
    x5: i32,
    x6: i32,
    x7: i32,
    x8: i32,
}
impl Foo {
    pub fn new() -> Self {
        Self {
            x: String::from("Hello"),
            x1: 0,
            x2: 0,
            x3: 0,
            x4: 0,
            x5: 0,
            x6: 0,
            x7: 0,
            x8: 0,
        }
    }
}

fn main() {
    let x: Foo = Foo::new();

    println!("{}", x.x);
}
