#[derive(Debug)]
struct E {
    a: String,
}

impl Drop for E {
    fn drop(&mut self) {
        println!("destroyed struct E");
    }
}

fn fn_once<F>(func: F)
where
    F: FnOnce(),
{
    println!("fn_once begins");
    func();
    //func();
    println!("fn_once ended");
}

pub fn fn_once_test() {
    let e = E {
        a: "fn_once".to_string(),
    };
    // 这样加个move，看看程序执行输出顺序有什么不同
    // let f = move || println!("fn once calls: {:?}", e);
    let f = || println!("fnonce closure calls: {:?}", e);
    fn_once(f);
    println!("main ended");
}
