pub fn hello() -> Result<(), String> {
    println!("welcome to rust world!");
    Err("Cannot hello".into())
}

