#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

#[derive(PartialEq, Debug)]
enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}
impl Day {
    fn is_weekend(&self) -> bool {
        *self == Day::Saturday || *self == Day::Sunday
    }

    fn is_monday(&self) -> bool {
        *self == Day::Monday
    }
}
impl Display for Day {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Day::Sunday => write!(f, "Weekend"),
            Day::Monday => write!(f, "Fuck Monday"),
            Day::Tuesday => write!(f, "Tacos Taste Better"),
            Day::Wednesday => write!(f, "Hump Day"),
            Day::Thursday => write!(f, "Pay Day"),
            Day::Friday => write!(f, "Weekend Lite"),
            Day::Saturday => write!(f, "Weekend"),
        }
    }
}

fn main() {
    let today: Day = Day::Monday;
    println!("{}, {:?}", today, today);

    println!("Is today the weekend? {}", today.is_weekend());

    if today.is_monday() {
        println!("It's fucking monday goddamnit.");
    }
}
