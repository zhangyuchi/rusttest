use std::error::Error;
use std::result::Result;

fn get_elem(array: &[i32], i: usize) -> i32 {
    return array[i];
}

pub fn array_test() -> Result<(), Box<dyn Error>> {
    println!("arraytest array_test");

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    println!("err element of the array: {}", get_elem(&xs, 6));

    Ok(())
}
