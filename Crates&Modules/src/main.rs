#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
mod restaurant;
use crate::restaurant::order_food;

fn main() {
    //  Crates : Modules that produce a library
    //  or executable.
    //  Modules : Organize and handle privacy.
    //  Packages : Build, test, and shar crates.
    //  Paths : A way of naming an item such as
    //  a struct or function.

    order_food();
}
