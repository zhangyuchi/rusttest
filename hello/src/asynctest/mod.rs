use futures::executor::block_on;

mod asyncfunc;

pub fn run_test() {
    block_on(asyncfunc::async_main()); // `future` is run and "hello, world!" is printed
}
