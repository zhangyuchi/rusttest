#[derive(Debug)]
struct E {
    a: String,
}

impl E {
    fn errinfo(&self) -> String{
        return self.a.clone();
    }
}

impl Drop for E {
    fn drop(&mut self) {
        println!("destroyed struct E");
    }
}

fn fn_func<F>(func: F)
where
    F: Fn() -> String,
{
    println!("fn begins");
    func();
    func();
    println!("fn ended");
}

pub fn fn_test() {
    let e = E {
        a: "fn".to_string(),
    };
    let f = move || { println!("fn once calls: {:?}", e); e.errinfo() }; //把e从外部move到内部，无法在外部继续使用e，在f内部，因为e没有发生move，闭包内的e一直有效
    //let f = || { println!("fn once calls: {:?}", e); e.a }; //闭包结束处，e发生了move，所以此时闭包内的e已经无效，是FnOnce类型
    //let f = ||{ println!("fn closure calls: {:?}", e); e.errinfo() };
    fn_func(f);

    //println!("E = {:?}", e); //当32行，把e从外部move到内部，就无法在外部继续使用e
    println!("main ended");
}
