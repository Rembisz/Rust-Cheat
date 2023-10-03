#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    const PI: f32 = 3.14159265359879323; // Pi needed for later calculation
    struct Customer {
        name: String,
        address: String, // Struct layout
        balance: f32,
    }
    let mut bob = Customer {
        name: String::from("Bob Smith"),
        address: String::from("555 Main St"), // Fill in data
        balance: 234.50,
    };
    bob.address = String::from("505 Main St"); // Replace data

    struct Rectangle<L, H> {
        length: L,
        height: H, // Generics
    }
    let rec = Rectangle {
        length: 4,
        height: 10.5,
    };

    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32; // Trait that, when implied for a struct, collects dimensions and calculates area
    }
    struct Circle {
        length: f32,
        width: f32,
    };
    struct Square {
        length: f32,
        width: f32,
    };

    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle { length, width };
        }
        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI; // Area of a circle
        }
    }
    impl Shape for Square {
        fn new(length: f32, width: f32) -> Square {
            return Square { length, width };
        }
        fn area(&self) -> f32 {
            return (self.length).powf(2.0); // Area of a square
        }
    }
    let circ: Circle = Shape::new(10.0, 10.0); // Data assignment
    let squa: Square = Shape::new(10.0, 10.0);
    println!("Circle Area : {}", circ.area());
    println!("Square Area : {}", squa.area());
}
