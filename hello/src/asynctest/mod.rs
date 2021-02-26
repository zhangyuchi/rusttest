use futures::executor::block_on;

mod asyncfunc;
mod asynclamda;
mod asynctokio;
//mod oldtokio;

pub fn run_test() {
  block_on(asyncfunc::async_main()); // `future` is run and "hello, world!" is printed
  asynclamda::async_main();
  asynctokio::async_main();
  //oldtokio::pool_main();
}
