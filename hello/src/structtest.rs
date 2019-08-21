use std::fmt;

#[derive(Debug)]
struct Animal {
    pub eye: bool,
    pub ear: bool,
    pub foot: bool,
}

impl fmt::Display for Animal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{{{0},{1},{2}}}", self.eye, self.ear, self.foot)
    }
}

#[derive(Debug)]
pub struct Cat(Animal);

// Implement `Display` for `MinMax`.
impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({})", self.0)
    }
}

pub fn print_info() {
    println!("structtest print_info");

    let cat = Cat(Animal {
        eye: true,
        ear: true,
        foot: true,
    });

    print!("animal is {}\n", cat);
    print!("animal is {:?}\n", cat);
}
