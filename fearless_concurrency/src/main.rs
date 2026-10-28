use std::{
    sync::{Arc, Mutex, mpsc},
    thread::{self, *},
    time::Duration,
};
//Its may be a good idea to just // everything in this chapter
fn main() {
    // let handle = spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {i} from the spawned thread!");
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    // handle.join().unwrap();
    // for i in 1..5 {
    //     println!("hi number {i} from the main thread!");
    //     thread::sleep(Duration::from_millis(1));
    // }
    // handle.join().unwrap();
    // let v = vec![1, 2, 3];

    // let handle = spawn(move || {
    //     println!("Here's a vector: {v:?}");
    // });
    // handle.join().unwrap();
    //transmitter / reveiver
    // let (tx, rx) = mpsc::channel();
    // let tx1 = tx.clone();
    // spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    // spawn(move || {
    //     let vals = vec![
    //         String::from("more"),
    //         String::from("message"),
    //         String::from("for"),
    //         String::from("you"),
    //     ];

    //     for val in vals {
    //         tx1.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    // for received in rx {
    //     println!("Got: {received}");
    // }

    // let m = Mutex::new(5);
    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }
    // println!("m={m:?}");

    // let counter = Mutex::new(0);
    // let mut handles = vec![];
    // for _ in 0..10 {
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // println!("Result: {}", *counter.lock().unwrap());

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
