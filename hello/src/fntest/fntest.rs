#[derive(Debug)]
struct E {
    a: String,
}

impl Drop for E {
    fn drop(&mut self) {
        println!("destroyed struct E");
    }
}

fn fn_func<F>(func: F)
where
    F: Fn(),
{
    println!("fn begins");
    func();
    println!("fn ended");
}

pub fn fn_test() {
    let e = E {
        a: "fn".to_string(),
    };
    // 这样加个move，看看程序执行输出顺序有什么不同
    // let f = move || println!("fn once calls: {:?}", e);
    let f = move || println!("fn closure calls: {:?}", e);
    fn_func(f);
    println!("main ended");
}
