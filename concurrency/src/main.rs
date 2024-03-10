use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..=10 {
            println!("Thread{:?}: {}", thread::current().id(), i);
            thread::sleep(Duration::from_secs(1))
        }
    });
    for i in 0..=10 {
        println!("Thread{:?}: {}", thread::current().id(), i);
        thread::sleep(Duration::from_millis(100))
    }
    handle.join().unwrap();
    let v1 = vec![3, 4, 5, 6, 1, 2, 4, 5, 6];
    let v2 = vec![3, 4, 5, 6, 1, 2, 4, 5, 6];
    let handle1 = thread::spawn(move || {
        for number in v1 {
            println!("Thread {:?}: number {}", thread::current().id(), number);
            thread::sleep(Duration::from_millis(100))
        }
    });
    let handle2 = thread::spawn(move || {
        for number in v2 {
            println!("Thread {:?}: number {}", thread::current().id(), number);
            thread::sleep(Duration::from_millis(100))
        }
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
}
