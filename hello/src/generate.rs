use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

pub fn fib() {
  let mut generator = || {
    let mut curr: u32 = 1;
    let mut next: u32 = 1;
    loop {
      let new_next = curr.checked_add(next);
      if let Some(new_next) = new_next {
        curr = next;
        next = new_next;
        yield curr; // <-- 新的关键字
      } else {
        return;
      }
    }
  };

  loop {
    //unsafe {
    match Pin::new(&mut generator).resume(()) {
      GeneratorState::Yielded(v) => println!("{}", v),
      GeneratorState::Complete(_) => return,
    }
    //}
  }
}

pub fn fac() {
  let mut generator = || {
    let mut curr: u32 = 1;
    let mut res: u32 = 1;
    loop {
      let new_res = curr.checked_mul(res);
      if let Some(new_res) = new_res {
        curr += 1;
        res = new_res;
        yield res; // <-- 新的关键字
      } else {
        return;
      }
    }
  };

  loop {
    //unsafe {
    match Pin::new(&mut generator).resume(()) {
      GeneratorState::Yielded(v) => println!("{}", v),
      GeneratorState::Complete(_) => return,
    }
    //}
  }
}

pub fn generate() {
  fib();
  println!("+++++++++++++++++++++++++++++++++++++");
  fac();
}
