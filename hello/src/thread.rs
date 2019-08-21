use std::sync::mpsc;
use std::thread;
use std::time::Duration;
//use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn run() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn channel() {
    println!("thread channel");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); //val所有权转移
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

pub fn vec_channel() {
    println!("thread vec_channel");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        //println!("vals: {:?}", vals);
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn lock_run() {
    println!("thread lock_run");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

pub fn run_test() {
    run();
    channel();
    vec_channel();
    lock_run();
    ()
}
