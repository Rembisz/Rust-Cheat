#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let age = rand::thread_rng().gen_range(1..120);
    if (age >= 1) && (age <= 18) {
        println!("{} is an important birthday.", age);
    } else if (age == 21) || (age == 50) {
        println!("{} is an important birthday.", age);
    } else if age >= 65 {
        println!("{} is an important birthday.", age);
    } else {
        println!("{} is not an important birthday.", age);
    }
    let can_vote = if age >= 18 { true } else { false };
    println!("Can Vote : {}", can_vote);
}
