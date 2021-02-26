//use tokio_threadpool::{ThreadPool, blocking};
use tokio::runtime::Runtime;
//use tokio::runtime::thread_pool::ThreadPool;

use oldfutures::future::{lazy, poll_fn};
use oldfutures::Future;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

//https://rustlang-cn.org/office/rust/async-rust/async_await/chapter.html

pub fn pool_main() {
  // This is a *blocking* channel
  let (tx, rx) = mpsc::channel();

  // Spawn a thread to send a message
  thread::spawn(move || {
    thread::sleep(Duration::from_millis(500));
    tx.send("hello").unwrap();
  });

  let mut rt = Runtime::new().unwrap();
  rt.blocking_pool(lazy(move || {
    // Because `blocking` returns `Poll`, it is intended to be used
    // from the context of a `Future` implementation. Since we don't
    // have a complicated requirement, we can use `poll_fn` in this
    // case.
    poll_fn(move || {
      spawn_blocking(|| {
        let msg = rx.recv().unwrap();
        println!("message = {}", msg);
      })
      .map_err(|_| panic!("the threadpool shut down"))
    })
  }));

  // Wait for the task we just spawned to complete.
  pool.shutdown_on_idle().wait().unwrap();
}
