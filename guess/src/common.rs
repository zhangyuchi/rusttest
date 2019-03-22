//use std::collections::hash_map::Entry::Occupied;

pub fn hello() -> Result<(), String> {
    let mut s = String::from("hello");

    let mut r3 = &s; // BIG PROBLEM
    let r1 = &s; // no problem
    let r2 = &s; // no problem

    println!("{}, {}, and {}", r1, r2, r3.append);
    //Err("Cannot hello".into());
    return Ok(());
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}