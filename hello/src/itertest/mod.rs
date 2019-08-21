mod arraytest;
mod listtest;
mod vectest;

pub fn run_test() {
    match vectest::vec_test() {
        Ok(()) => (),
        Err(e) => println!("Error: {}", e),
    }

    listtest::list_test().expect("list test error");
    arraytest::array_test().expect("array test error")
}
