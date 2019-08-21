use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

pub fn generate() {
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
        match Pin::new(&mut generator).resume() {
            GeneratorState::Yielded(v) => println!("{}", v),
            GeneratorState::Complete(_) => return,
        }
        //}
    }
}
