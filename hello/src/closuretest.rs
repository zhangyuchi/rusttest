struct Person {
    name : String,
    age : u8,
}

pub fn test() {
    test_lifetime();
    test_return();
    test_borrow();
    test_thread();
    test_thread_shared();
}

fn test_lifetime() {
    let p = Person{ name: "Hao Chen".to_string(), age : 44};
    //可以运行，因为 `u8` 有 Copy Trait
    let age = |p : &Person| p.age;
    // String 没有Copy Trait，所以，这里所有权就 Move 走了
    //let name = |p : &Person | &p.name;
    //下面的声明可以正确译
    let name: for<'a> fn(&'a Person) -> &'a String = |p: &Person| &p.name; //closure include lifetime vars
    println! ("name={}, age={}" , name(&p), age(&p));
}

fn test_return() {
    let s = String::from("coolshell");
    let take_str = || s;
    //println!("{}", s); //ERROR
    println!("{}",  take_str()); // OK
}

fn test_borrow() {
    //-----------借用的情况-----------
    let mut num = 5;
    {
        let mut add_num = |x: i32| num += x;
        add_num(5);
    }
    println!("num={}", num); //输出 10
//-----------Move的情况-----------
    let mut num2 = 5;
    {
        // 把 num2（5）所有权给 move 到了 add_num 中，
        // 使用其成为闭包中的局部变量。
        let mut add_num = move |x: i32| {num2 += 0; println!("num2(move) begin ={}", num2); num2 += x; println!("num2(move) after ={}", num2); num2};
        println!("num(move) result ={}", add_num(5)); //输出10
    }
//因为i32实现了 `Copy`，所以，这里还可以访问
    println!("num(move)={}", num2); //输出5
}

use std::thread;

fn test_thread() {
    let name = "rust thread".to_string();
    let name1 = name.clone();

    let t1 = thread::spawn(move || {
        println!("Hello, {}", name.clone());
    });

    let t2 = thread::spawn(move || {
        //println!("Hello, {}", name.clone()); ERROR
    });

    println!("wait t1={:?}, t2={:?}", t1.join(), t2.join());
}

use std::sync::Arc;
use std::sync::atomic::{Ordering, AtomicU64};

fn test_thread_shared() {
    const TOTAL_SIZE:usize = 100 * 1000; //数组长度
    const NTHREAD:usize = 6; //线程数

    let data : Vec<i32> = (1..(TOTAL_SIZE+1) as i32).collect(); //初始化一个数据从1到n数组
    let arc_data = Arc::new(data); //data 的所有权转给了 ar_data
    let result  = Arc::new(AtomicU64::new(0)); //收集结果的数组(原子操作)

    let mut thread_handlers = vec![]; // 用于收集线程句柄

    for i in 0..NTHREAD {
        // clone Arc 准备move到线程中，只增加引用计数，不会深拷贝内部数据
        let test_data = arc_data.clone();
        let res = result.clone();
        thread_handlers.push(
            thread::spawn(move || {
                let id = i;
                //找到自己的分区
                let chunk_size = TOTAL_SIZE / NTHREAD + 1;
                let start = id * chunk_size;
                let end = std::cmp::min(start + chunk_size, TOTAL_SIZE);
                //进行求和运算
                let mut sum = 0;
                for  i in start..end  {
                    sum += test_data[i];
                }
                //原子操作
                res.fetch_add(sum as u64, Ordering::SeqCst);
                println!("id={}, sum={}", id, sum );
            }
            ));
    }
//等所有的线程执行完
    for th in thread_handlers {
        th.join().expect("The sender thread panic!!!");
    }
//输出结果
    println!("result = {}",result.load(Ordering::SeqCst));
}
