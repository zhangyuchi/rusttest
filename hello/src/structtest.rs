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

struct Buffer {
  buffer: String,
}

struct Render {
  current_buffer: Buffer,
  next_buffer: Buffer,
}
//实现结构体 `Render` 的方法
use std::mem::replace;

impl Render {
  //实现 update_buffer() 方法，
  //更新buffer，把 next 更新到 current 中，再更新 next
  // compile error
  // fn update_buffer(&mut self, buf: String) {
  //   self.current_buffer = self.next_buffer; //ERROR
  //   self.next_buffer = Buffer { buffer: buf };
  // }

  //compile ok
  fn update_buffer2(&mut self, buf: String) {
    self.current_buffer = replace(&mut self.next_buffer, Buffer { buffer: buf });
  }
}

impl fmt::Display for Render {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // Use `self.number` to refer to each positional data point.
    write!(
      f,
      "{{{0},{1}}}",
      self.current_buffer.buffer, self.next_buffer.buffer
    )
  }
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

  let render = Render {
    current_buffer: Buffer {
      buffer: String::from("current"),
    },
    next_buffer: Buffer {
      buffer: String::from("next"),
    },
  };

  print_animal(dog.clone());
  print!("animal is {}\n", dog.animal);
  print!("animal is {:?}\n", dog.animal);

  print!("render is {}\n", render)
}
