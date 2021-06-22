#![feature(generators, generator_trait)]
#![feature(async_closure)]

//feature需要加到main.rs或者lib.rs
mod asynctest;
mod collectiontest;
mod fntest;
mod generate;
mod itertest;
mod smart;
mod stringtest;
mod structtest;
mod template;
mod thread;
mod traittest;
mod closuretest;
mod ootest;

use clap::{App, Arg};

fn default(name: &str) {
  print!("unsupport type arg: {}", name)
}

fn main() {
  let command = App::new("rusttest")
    .version("0.1.0")
    .author("zhangjun")
    .about("Learn use Rust")
    .arg(
      Arg::with_name("verbose")
        .short("v")
        .multiple(true)
        .help("verbosity level"),
    )
    .args_from_usage("-t --type=[Test Type] 'test type'")
    .get_matches();

  let type_arg = command.value_of("type").expect("need type arg");

  match type_arg {
    "thread" => thread::run_test(),
    "smart" => smart::run_test(),
    "template" => template::largest_run(),
    "trait" => traittest::print_person(),
    "struct" => structtest::print_info(),
    "iter" => itertest::run_test(),
    "gen" => generate::generate(),
    "async" => asynctest::run_test(),
    "fn" => fntest::fn_test(),
    "string" => stringtest::test(),
    "collect" => collectiontest::test(),
    "closure" => closuretest::test(),
    "oo" => ootest::test(),
    _ => default(type_arg),
  }
}
