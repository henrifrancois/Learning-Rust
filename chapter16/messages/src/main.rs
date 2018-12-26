// mpsc: multiple producer single consumer
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
/*
fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("got: {}", received);
}
 */
/*
fn main() {
    let (tx, rx) = mpsc::channel();

    let first = thread::spawn(move || {
        let val = String::from("from thread 1");
        println!("in thread 1");;
        tx.send(val).unwrap();
    });

    let second = thread::spawn(move || {
        let received = rx.recv().unwrap();
        println!("in thread 2 \nReceived: {}", received);
    });

    second.join().unwrap();
    println!("main thread running");
}
 */

fn main(){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let values = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];

        for value in values {
            tx.send(value).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Received -> {}", received);
    }
}
