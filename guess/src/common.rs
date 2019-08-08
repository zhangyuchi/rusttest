//use std::collections::hash_map::Entry::Occupied;
fn string_slice(arg: &str) -> &str {
    println!("{}", arg);
    arg
}

fn string(arg: String) -> String{
    println!("{}", arg);
    arg
}

pub fn hello() -> Result<(), String> {
    let mut s = String::from("hello");

    let r1 = &mut s; // BIG PROBLEM
    let r2 = string_slice("tiny".into()); // no problem
    r1.push_str(" world");

    println!("{} and {}", r1, r2);
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