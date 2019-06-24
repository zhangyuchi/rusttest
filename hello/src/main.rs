#![feature(generators, generator_trait)]

//feature需要加到main.rs或者lib.rs
mod thread;

fn main() {
    thread::lock_run();
}
