#![feature(generators, generator_trait)]

//feature需要加到main.rs或者lib.rs
mod smart;
mod structtest;
mod template;
mod thread;
mod traittest;

fn main() {
    thread::lock_run();
    smart::refcell();

    template::largest_run();

    traittest::print_person();

    structtest::print_info();
}
