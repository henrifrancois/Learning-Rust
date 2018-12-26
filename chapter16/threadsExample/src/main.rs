use std::thread;
use std::time::Duration;

/* first example
fn main() {
    let handle = thread::spawn(|| {
        for integer in 1..10 {
            println!("hi number {} from the spawned thread", integer);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
    for integer in 1..5 {
        println!("hi number {} from the main thread", integer);
        thread::sleep(Duration::from_millis(1));
    }


}
*/


fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
