#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

// Heap : When putting data on the heap you request a
// certain amount of space. The kernel finds space available
// and returns an address for the space called a pointer.
//
// RULES
//    1. Each value has a variable that's called its owner
//    2. There is only one owner at a time
//    3. When the owner goes out of scope the value disappears

fn print_str(x: String) {
    println!(r#"A string "{}""#, x); // r#""# allows quotes to be evaluated as part of the raw string
}

fn print_return_str(x: String) -> String {
    println!(r#"A string "{}""#, x);
    x
}

fn change_string(name: &mut String) {
    name.push_str(" is happy");
    println!("Message : {}", name);
}

fn main() {
    let str1 = String::from("Hello");
    let str2 = str1;
    //  println!("{} World!", str1); will no longer work.
    let str3 = String::from("World!");
    let str4 = str3.clone(); // Creates a duplicate value with the same owner
    println!("Hello {}", str3);
    println!("Hello {}", str4);

    print_str(str2);
    let str5 = print_return_str(str4);
    println!("str5 = {}", str5);
    let mut my_name = String::from("Matt");
    change_string(&mut my_name);
}
