#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let vec1: Vec<i32> = Vec::new(); // Create undefined vector
    let mut vec2 = vec![1, 2, 3, 4]; // Create defined vector
    vec2.push(5); // Add value to end of vector (must be same type)
    println!("1st : {}", vec2[0]); // Print specific vector index
    let second: &i32 = &vec2[1]; //
    match vec2.get(1) {
        Some(second) => println!("2nd : {}", second),
        None => println!("No 2nd value"), // Verify a value exists
    }
    for i in &mut vec2 {
        *i *= 2; // Multiply each index by 2
    }
    for i in &vec2 {
        println!("{}", i); // Print each index
    }
    println!("Vec Length {}", vec2.len());
    println!("Pop : {:?}", vec2.pop()); // Remove AND return last index
}
