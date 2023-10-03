#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let age = rand::thread_rng().gen_range(1..120);
    match age {
        1..=18 => println!("{} is an important birthday.", age),
        21 | 50 => println!("{} is an important birthday.", age),
        65..=i32::MAX => println!("{} is an important birthday.", age),
        _ => println!("{} is not an important birthday.", age),
    };

    let voting_age = 18;
    match age.cmp(&voting_age) {
        Ordering::Less => println!("Cannot vote."),
        Ordering::Greater => println!("Can vote."),
        Ordering::Equal => println!("New voter."),
    };
}
