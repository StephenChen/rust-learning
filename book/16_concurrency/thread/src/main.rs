use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    spawn();
}

fn spawn() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number {} from the spawned thread.", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("number {} from the main thread.", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let v = vec![1, 2, 3];

    let handle1 = thread::spawn(move || {
        println!("vector: {:?}", v);
    });

    handle1.join().unwrap();
}
