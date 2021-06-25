mod list;
mod rbtree;
mod graph;

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
        "list" => list::test(),
        "rbtree" => rbtree::test(),
        "graph" => graph::test(),
        _ => default(type_arg),
    }
}