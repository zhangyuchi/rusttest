mod single_list;
mod double_list;

pub fn test() {
    test_single();
    test_double();
}

fn test_single() {
    let mut newlist = single_list::init_list::<i32>();
    newlist.push(30);
    newlist.push(3);
    newlist.push(10);
    newlist.push(20);
    newlist.push(1);
    println!("test single");
    println!("list: {:?}", newlist);
    println!("back: {:?}", newlist.back());
}

fn test_double() {
    let mut newlist = double_list::init_list::<i32>();
    newlist.push(30);
    newlist.push(3);
    newlist.push(10);
    newlist.push(20);
    newlist.push(1);
    println!("test double");
    println!("front: {:?}", newlist.front());
    println!("back: {:?}", newlist.back());
}