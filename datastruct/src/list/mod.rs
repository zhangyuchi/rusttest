mod container;

pub fn test() {
    let newlist = container::init_list::<i32>();
    println!("list: {:?}", newlist)
}