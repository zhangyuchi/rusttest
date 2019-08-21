use std::error::Error;
use std::result::Result;

pub fn vec_test() -> Result<(), Box<dyn Error>> {
    println!("vectest vec_test");
    //let v1 = vec![1, 2, 3];

    print!("-------------iter--------------\n");

    let mut v1 = vec![1, 2, 3];

    for e in v1.iter_mut() {
        println!("{:?}", *e);
        *e += 2;
    }

    print!("\n");

    for e in v1.iter() {
        println!("{:?}", *e)
    }

    print!("-------------fold--------------\n");
    let r = v1
        .iter()
        .fold(1, |acc, e| if *e < 4 { acc * *e * 2 } else { acc * *e });
    println!("result = {}", r);

    print!("-------------map--------------\n");
    let r1: Vec<_> = v1.iter().map(|e| *e * 2).collect();
    println!("result = {:#?}", r1);

    print!("-------------filter--------------\n");
    let r2: Vec<_> = v1.iter().filter(|e| *e % 2 == 0).collect();
    println!("result = {:#?}", r2);

    Ok(())
}
