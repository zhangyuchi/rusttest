use std::error::Error;
use std::result::Result;

pub fn list_test() -> Result<(), Box<dyn Error>> {
    println!("listtest list_test");
    Ok(())
}
