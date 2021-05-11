use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
fn spawn() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        for _ in 1..10 {
            println!("Here's a vector: {:?}", v);
            //println!("hi number {} from the spawned thread!", i);
            //thread::sleep(Duration::from_millis(1));
        }
    });
    //handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}


fn channel_thread<'a>(msgs: Vec<&'a str>, millis: u64, tx: mpsc::Sender<&'a str>) {
    for msg in msgs {
        tx.send(msg).unwrap();
        println!("Sent: {}", msg);
        thread::sleep(Duration::from_millis(millis));
    }
}

#[allow(dead_code)]
fn channel() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let msgs = vec!("Hi!", "How", "is", "tricks?");
        channel_thread(msgs, 100, tx);
    });
    thread::spawn(move || {
        let msgs = vec!("Things", "alright?");
        channel_thread(msgs, 500, tx1);
    });

    for msg in rx {
        println!("Got: {}", msg);
    }
}


fn mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Num: {}", num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}


pub fn run() {
//    spawn();
//    channel();
    mutex();
}