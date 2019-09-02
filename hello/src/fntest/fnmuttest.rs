#[derive(Debug)]
struct E {
    a: String,
}

impl Drop for E {
    fn drop(&mut self) {
        println!("destroyed struct E");
    }
}

fn fnmul_func<F>(mut func: F)
where
    F: FnMut(),
{
    println!("fnmut begins");
    func();
    println!("fnmut ended");
}

pub fn fnmul_test() {
    let e = E {
        a: "fn".to_string(),
    };
    // 这样加个move，看看程序执行输出顺序有什么不同
    // let f = move || println!("fn once calls: {:?}", e);
    let f = move || println!("fnmut closure calls: {:?}", e);
    fnmul_func(f);
    println!("main ended");
}
