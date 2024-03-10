use std::{
    sync::{Arc, Mutex},
    thread, vec,
};
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
fn main() {
    let mutex_arr = Arc::new(Mutex::new(Person {
        name: String::from(""),
        age: 0,
    }));
    let mut handles = vec![];
    {
        let mutex_arr_clone = Arc::clone(&mutex_arr);
        let handle = thread::spawn(move || {
            let mut arr = mutex_arr_clone.lock().unwrap();
            arr.name = String::from("aaaaaaa")
        });
        handles.push(handle)
    }
    {
        let mutex_arr_clone = Arc::clone(&mutex_arr);
        let handle = thread::spawn(move || {
            let mut arr = mutex_arr_clone.lock().unwrap();
            arr.age = 30
        });
        handles.push(handle)
    }
    for handle in handles {
        handle.join().unwrap()
    }
    println!("{:?}", *mutex_arr.lock().unwrap());
}
