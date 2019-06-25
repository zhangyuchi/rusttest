#![feature(generators, generator_trait)]

//feature需要加到main.rs或者lib.rs
mod thread;
mod smart;
mod template;

fn main() {
    thread::lock_run();
    smart::refcell();

    template::largest_run();
}
