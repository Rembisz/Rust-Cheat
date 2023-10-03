#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");
    for word in st1.split_whitespace() {
        println!("{}", word);
    }
    println!("{}", st1);

    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    let st2 = String::from("x g p l v v a b c");
    let mut v1: Vec<char> = st2.chars().collect(); // string to vector
    v1.sort(); // sort v1 alphabetically
    v1.dedup(); // remove duplicate chars
    for char in v1 {
        println!("{}", char);
    }
    let st3: &str = "Random string";
    let mut st4: String = st3.to_string(); // convert &str to String type
    println!("{}", st4);

    let byte_arr = st4.as_bytes(); // converts st4 to byte slice (array)

    let st5 = &st4[0..6];
    println!("String length : {}", st5.len()); // returns st5's length, which is comprised of index 0-6 of st4

    st4.clear(); // *IF MUTABLE* clears data in a string

    let st6 = String::from("Just some");
    let st7 = String::from(" words");
    let st8 = st6 + &st7; // due to "&", st7 still exists after execution. st6 no longer exists and is now st8
    println!("{}", st8);

    for char in st8.bytes() {
        println!("{}", char); // returns byte value for each char in st8 via for loop
    }
}
