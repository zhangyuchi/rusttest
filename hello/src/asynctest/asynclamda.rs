use futures::executor::block_on;
use futures::Future;
use std::thread;
use std::time::Duration;

fn compute_01() -> impl Future<Output = Result<(), String>> {
    let f = async move {
        for i in 1..10000 {
            let _ = i * 2;
        }
        println!("=>01 it is over!");
        Ok(())
    };

    return f;
}

async fn compute_02() {
    println!("=>02 is entering....");
    match compute_01().await {
        Ok(()) => println!("01 success"),
        Err(errinfo) => println!("01 error {}", errinfo),
    };
    //compute_04().await;
    println!("=>02 is over!");
}

async fn compute_03() {
    println!("=>03 is entering....");
    for i in 1..10000 {
        let _ = i * 2;
    }
    println!("=>03 it is over!");
}

fn compute_05(value: i32) -> impl Future<Output = i32> {
    let closure = async move |v: i32| {
        compute_03().await;
        v + 1
    };
    closure(value)
}

pub fn async_main() {
    block_on(compute_02());
    //compute_03();
    block_on(compute_03()); //必须把这些异步跑完

    let val = block_on(compute_05(6));
    println!("val :{:?}", val);
    println!("hello world!");
    thread::sleep(Duration::from_millis(500));
}
