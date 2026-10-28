use std::{os::windows::thread, time::Duration};
use trpl::StreamExt;

fn main() {
    trpl::block_on(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);
        while let Some(value) = stream.next().await {
            println!("the value was : {value}");
        }
    });
    let (tx, mut rx) = trpl::channel();
    std::thread::spawn(move || {
        for i in 1..1 {
            tx.send(i).unwrap();
            std::thread::sleep(Duration::from_secs(1));
        }
    });
    trpl::block_on(async {
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
    });
}
