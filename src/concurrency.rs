use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;
use std::sync::Arc;

fn main() {
    let handle = thread::spawn(|| {
        println!("new thread started...");
        for i in 1..10 {
            println!("number from thread {}", i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 1..5 {
        println!("number from main thread {}", i);
        thread::sleep(Duration::from_secs(1));
    }
    println!("waiting for all threads to complete...");
    handle.join().unwrap();
    println!("exiting...");

    let list = vec![1, 2, 3];
    let handle = thread::spawn(move || { // list may be dropped before the thread finishes, need to give ownership to the closure
        println!("{:?}", list);
    });

    //drop(list);
    handle.join().unwrap();

    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(5));
        sender.send("Hello Msg").unwrap();
    });

    println!("waiting for msg...");
    let msg = receiver.recv().unwrap();
    println!("message received {}", msg);

    let (s, r) = mpsc::channel();
    let s1 = s.clone();
    let s2 = s.clone();
    thread::spawn(move || {
        let orders = vec![
            "order1",
            "order2",
            "order3"
        ];

        for order in orders {
            s1.send(order).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let orders = vec![
            "order4",
            "order5",
            "order6"
        ];
        for order in orders {
            s2.send(order).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    thread::spawn(move || {
        for order in r {
            println!("order received => {}", order);
        }
    });

    let shared_number = Mutex::new(1);
    {
        let mut n = shared_number.lock().unwrap();
        *n = 2;
    }
    println!("number is {:?}", shared_number);

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
    println!("counter => {}", counter.lock().unwrap());
}