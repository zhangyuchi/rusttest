use std::error::Error;
use std::result::Result;

#[derive(Debug, Clone)]
pub struct Pod {
    id: i32,
    name: String,
}

fn get_clone_elem<T: Clone>(list: &[T], i: usize) -> T {
    //T是PartialOrd类型，才可以比较大小
    let val = list[i].clone(); //max是一个可变引用
    val
}

fn get_nocopy_elem<T>(list: &[T], i: usize) -> &T {
    //T是PartialOrd类型，才可以比较大小
    let val = &list[i]; //max是一个可变引用
    val
}

fn get_copy_elem<T: Copy>(list: &[T], i: usize) -> T {
    //T是PartialOrd类型，才可以比较大小
    let val = list[i]; //max是一个可变引用
    val
}

fn get_base_elem(array: &[i32], i: usize) -> i32 {
    let val = array[i]; //max是一个可变引用
    val
}

pub fn array_test() -> Result<(), Box<dyn Error>> {
    println!("arraytest array_test");

    let ab: [i32; 5] = [1, 2, 3, 4, 5];
    let mut i = 3;
    println!("element of the array[{}]: {}", i, get_base_elem(&ab, i));

    i = 1;
    println!("element of the array[{}]: {}", i, get_copy_elem(&ab, i));

    let ag: [Pod; 1] = [Pod {
        id: 1,
        name: String::from("123"),
    }];
    i = 0;
    println!("element of the array[{}]: {:?}", i, get_nocopy_elem(&ag, i));

    println!("element of the array[{}]: {:?}", i, get_clone_elem(&ag, i));

    Ok(())
}
