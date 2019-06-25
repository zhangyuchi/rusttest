fn largest<T:PartialOrd>(list: &[T]) -> &T {//T是PartialOrd类型，才可以比较大小
    let mut max = &list[0];//max是一个可变引用

    for item in list.iter() { //item是一个不可变引用
        if item > max {
            max = item;
        }
    }
    max
}

pub fn largest_run() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
