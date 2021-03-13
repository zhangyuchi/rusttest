use std::fmt;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Dog {
  animal: Animal,
}

// Implement `Display` for `MinMax`.
impl fmt::Display for Dog {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // Use `self.number` to refer to each positional data point.
    write!(f, "({})", self)
  }
}

use std::fmt::Debug;
fn print_animal<T: Debug>(animal: T) {
  print!("animal is {:?}\n", animal);
}

pub fn print_info() {
  println!("structtest print_info");

  let cat = Cat(Animal {
    eye: true,
    ear: true,
    foot: true,
  });
  print_animal(cat);

  let dog = Dog {
    animal: Animal {
      eye: true,
      ear: true,
      foot: true,
    },
  };

  print_animal(dog.clone());
  print!("animal is {}\n", dog.animal);
  print!("animal is {:?}\n", dog.animal);
}
